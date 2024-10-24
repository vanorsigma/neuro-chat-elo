use std::{
    collections::HashMap, sync::{atomic::AtomicU32, Arc}
};

use _types::clptypes::{Message, MetadataTypes, MetadataUpdate, MetricUpdate, UserChatPerformance};
use log::{debug, warn};
use metadata::setup_metadata_and_channels;
use metrics::setup_metrics_and_channels;
use tokio::{
    sync::{ mpsc, broadcast, oneshot }, 
    task::JoinSet
};
use twitch_utils::{seventvclient::SevenTVClient, TwitchAPIWrapper};

pub mod _constants;
pub mod _types;
pub mod leaderboards;
pub mod metadata;
pub mod metrics;

/// Struct to setup a MessageProcessorRunning and spawn metric/metadata processors
/// 
/// Call .start() to spawn tasks and get a `MessageProcessorRunning` struct
pub struct MessageProcessorSetup {
    metric_processor: metrics::MetricProcessor,
    metric_sender: broadcast::Sender<(Message, u32)>,
    metric_receiver: mpsc::Receiver<MetricUpdate>,
    metadata_processor: metadata::MetadataProcessor,
    metadata_sender: broadcast::Sender<(Message, u32)>,
    metadata_receiver: mpsc::Receiver<MetadataUpdate>,
}

impl MessageProcessorSetup {
    pub async fn new(
        twitch: &TwitchAPIWrapper,
        seventv_client: Arc<SevenTVClient>,
    ) -> Self {
        let (metric_processor, metric_sender, metric_receiver) =
            setup_metrics_and_channels(seventv_client.clone());

        let (metadata_processor, metadata_sender, metadata_receiver) =
            setup_metadata_and_channels(twitch, seventv_client).await;

            MessageProcessorSetup {
            metric_processor,
            metric_sender,
            metric_receiver,
            metadata_processor,
            metadata_sender,
            metadata_receiver,
        }
    }

    pub async fn start(mut self) -> MessageProcessorRunning {
        let (performance_sender, performance_receiver) = oneshot::channel();

        let performances = performance_processor(
            self.metric_processor.defaults.clone(),
            self.metric_receiver,
            self.metadata_processor.defaults.clone(),
            self.metadata_receiver,
            performance_sender,
        );

        let mut joins = JoinSet::new();
        joins.spawn(async move { self.metric_processor.run().await });
        joins.spawn(async move { self.metadata_processor.run().await });
        joins.spawn(performances);

        debug!("Constructing MessageProcessorRunning");
        MessageProcessorRunning {
            joins,
            metric_sender: self.metric_sender,
            metadata_sender: self.metadata_sender,
            performance_receiver,
            sequence_number: AtomicU32::new(0)
        }
    }
}

/// A running message processor that can process messages
pub struct MessageProcessorRunning {
    joins: JoinSet<()>,
    metric_sender: tokio::sync::broadcast::Sender<(Message, u32)>,
    metadata_sender: tokio::sync::broadcast::Sender<(Message, u32)>,
    performance_receiver: tokio::sync::oneshot::Receiver<HashMap<String, UserChatPerformance>>,
    sequence_number: AtomicU32,
}

impl MessageProcessorRunning {
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

        self.joins.join_all().await;

        self.performance_receiver.await.expect("Received perfomances")
    }
}

pub async fn performance_processor(
    metric_defaults: HashMap<String, f32>,
    mut metric_receiver: mpsc::Receiver<MetricUpdate>,
    metadata_defaults: HashMap<String, MetadataTypes>,
    mut metadata_receiver: mpsc::Receiver<MetadataUpdate>,
    performance_sender: oneshot::Sender<HashMap<String, UserChatPerformance>>,
) {
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
    performance_sender.send(user_performances).unwrap();
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
