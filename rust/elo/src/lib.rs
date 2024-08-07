use std::{collections::HashMap, sync::atomic::AtomicU32};

use _types::clptypes::{Message, MetadataTypes, MetadataUpdate, MetricUpdate, UserChatPerformance};
use log::{debug, warn};
use metadata::setup_metadata_and_channels;
use metrics::setup_metrics_and_channels;
use tokio::sync::mpsc;

pub mod _constants;
pub mod _types;
pub mod leaderboards;
pub mod metadata;
pub mod metrics;

pub struct MessageProcessor {
    metric_processor_task: tokio::task::JoinHandle<()>,
    metric_sender: tokio::sync::broadcast::Sender<(Message, u32)>,
    metadata_processor_task: tokio::task::JoinHandle<()>,
    metadata_sender: tokio::sync::broadcast::Sender<(Message, u32)>,
    sequence_number: AtomicU32,
    performances_task: tokio::task::JoinHandle<HashMap<String, UserChatPerformance>>,
}

impl MessageProcessor {
    pub async fn new(twitch: &twitch_utils::TwitchAPIWrapper) -> Self {
        let (mut metric_processor, metric_sender, metric_receiver) =
            setup_metrics_and_channels().await;

        let (mut metadata_processor, metadata_sender, metadata_receiver) =
            setup_metadata_and_channels(twitch).await;

        let performances = user_chat_performance_processor(
            metric_processor.defaults.clone(),
            metric_receiver,
            metadata_processor.defaults.clone(),
            metadata_receiver,
        );

        Self {
            metric_processor_task: tokio::task::spawn(async move { metric_processor.run().await }),
            metric_sender,
            metadata_processor_task: tokio::task::spawn(
                async move { metadata_processor.run().await },
            ),
            metadata_sender,
            performances_task: tokio::task::spawn(performances),
            sequence_number: AtomicU32::new(0),
        }
    }

    pub async fn process_message(&self, message: Message) {
        let sequence_number = self
            .sequence_number
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        for sender in [&self.metric_sender, &self.metadata_sender] {
            sender.send((message.clone(), sequence_number)).unwrap();
        }
    }

    pub async fn finish(self) -> HashMap<String, UserChatPerformance> {
        // These senders need to be dropped before `metadata_processor_task`
        // and `metric_processor_task` will exit.
        drop(self.metric_sender);
        drop(self.metadata_sender);

        self.metadata_processor_task.await.unwrap();
        self.metric_processor_task.await.unwrap();

        self.performances_task.await.unwrap()
    }
}

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
