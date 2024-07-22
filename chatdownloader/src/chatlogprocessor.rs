use futures::join;
use log::{debug, info, warn};
use std::time::Instant;
use std::{collections::HashMap, fs};
use tokio::sync::broadcast;
use tokio::sync::mpsc;

use crate::_types::clptypes::{MetadataTypes, MetadataUpdate, MetricUpdate, UserChatPerformance};
use crate::_types::twitchtypes::{ChatLog, Comment};
use crate::twitch_utils::TwitchAPIWrapper;

use crate::leaderboards::LeaderboardProcessor;
use crate::metadata::setup_metadata_and_channels;
use crate::metrics::setup_metrics_and_channels;

pub struct ChatLogProcessor<'a> {
    /*
    Processes the chat logs.

    The class uses the metrics package to extract metrics from the
    chat messages, the metadata package to extract any user-metadata,
    and the leaderboards package to export the metrics / required user
    metadata to the right people
    */
    twitch: &'a TwitchAPIWrapper,
}

impl<'a> ChatLogProcessor<'a> {
    pub fn new(twitch: &'a TwitchAPIWrapper) -> Self {
        Self { twitch }
    }

    pub fn __parse_to_log_struct(&self, chat_log_path: String) -> ChatLog {
        let chat_log_str = fs::read_to_string(chat_log_path).unwrap();
        let chat_log: ChatLog = serde_json::from_str(&chat_log_str).unwrap();
        chat_log
    }

    pub async fn parse_from_log_object(&self, chat_log: ChatLog) -> Vec<UserChatPerformance> {
        let start_time = Instant::now();
        debug!("Starting chat log processing");

        debug!("Instantiated metric and metadata processors");

        debug!("Setting up channels for metric and metadata processors");
        let (mut metric_processor, metric_sender, metric_receiver) =
            setup_metrics_and_channels().await;

        let (mut metadata_processor, metadata_sender, metadata_receiver) =
            setup_metadata_and_channels(self.twitch).await;

        info!("Parsing chat log object");
        let chat_adder =
            chatlog_to_receiver(chat_log, vec![metric_sender, metadata_sender]);
        let performances = user_chat_performance_processor(metric_processor.defaults.clone(), metric_receiver, metadata_processor.defaults.clone(), metadata_receiver);

        let (_, _, _, performances) = join!(
            chat_adder,
            async move {metric_processor.run().await;},
            async move {metadata_processor.run().await;},
            performances,
        );
        info!(
            "Chat log processing took: {:#?}",
            start_time.elapsed()
        );
        performances.into_values().collect()
    }

    #[allow(dead_code)]
    async fn parse(&self, chat_log_path: String) -> Vec<UserChatPerformance> {
        let chat_log = self.__parse_to_log_struct(chat_log_path);
        self.parse_from_log_object(chat_log).await
    }

    /// A function to export the user performances to the leaderboards and save them
    pub async fn export_to_leaderboards(performances: Vec<UserChatPerformance>) {
        let mut leaderboard_processor = LeaderboardProcessor::new();
        leaderboard_processor.run(performances).await;
    }
}

/// A function to apwn a thread to take a ChatLog and add its comments to a receiver
pub async fn chatlog_to_receiver(
    chat_log: ChatLog,
    senders: Vec<broadcast::Sender<(Comment, u32)>>,
) {
    for (sequence_no, comment) in chat_log.comments.iter().enumerate() {
        for sender in senders.iter() {
            sender.send((comment.clone(), sequence_no as u32)).unwrap();
        }
        tokio::task::yield_now().await;
    }
    debug!("Finished adding comments to receivers");
}

/// A function to spawn a thread that takes two recievers and processes metrics / metadata from them and updates the user performances
pub async fn user_chat_performance_processor(
    metric_defaults: HashMap<String, f32>,
    mut metric_receiver: mpsc::Receiver<MetricUpdate>,
    metadata_defaults: HashMap<String, MetadataTypes>,
    mut metadata_receiver: mpsc::Receiver<MetadataUpdate>,
) -> HashMap<String, UserChatPerformance> {
    let mut user_performances: HashMap<String, UserChatPerformance> = HashMap::new();
    loop {
        tokio::select! {
            Some(metric_update) = metric_receiver.recv() => {
                metric_update.updates.iter().for_each(|(user_id, met_value)| {
                    let user_chat_performance = get_performance_or_default(&mut user_performances, user_id, &metric_defaults, &metadata_defaults);
                    user_chat_performance.metrics.entry(metric_update.metric_name.clone()).and_modify(|metric_value| *metric_value += met_value);
                });
            }
            Some(metadata_update) = metadata_receiver.recv() => {
                metadata_update.updates.iter().for_each(|(user_id, met_value)| {
                    let user_chat_performance = get_performance_or_default(&mut user_performances, user_id, &metric_defaults, &metadata_defaults);
                    match metadata_update.metadata_name.as_str() {
                        "basic_info" => {
                            if let Some((username, avatar)) = met_value.get_basic_info() {
                                user_chat_performance.username = username;
                                user_chat_performance.avatar = avatar;
                            } else {
                                warn!("Could not get username and/or url for user_id {}. Skipping", user_id);
                            }
                        }
                        _ => {
                            if let Some(metadata_value) = user_chat_performance.metadata.get_mut(&metadata_update.metadata_name) {
                                *metadata_value = met_value.clone();
                                debug!("Updating metadata: {} with value: {:?}", metadata_update.metadata_name, met_value);
                            }
                        }
                    }
                });
            }
            else => break,
        }
    }
    debug!("Finished processing user performances");
    user_performances
}

/// Get a user performance or create a new one if it doesn't exist
fn get_performance_or_default<'a>(
    user_performances: &'a mut HashMap<String, UserChatPerformance>,
    user_id: &'a str,
    metrics: &'a HashMap<String, f32>,
    metadatas: &'a HashMap<String, MetadataTypes>,
) -> &'a mut UserChatPerformance {
    user_performances
        .entry(user_id.to_owned())
        .or_insert(UserChatPerformance {
            id: user_id.to_owned(),
            username: user_id.to_owned(),
            avatar: "".to_string(),
            metrics: metrics.clone(),
            metadata: metadatas.clone(),
        })
}
