use std::{fs, collections::HashMap};
use std::time::Instant;
use log::{debug, info};
use serde_json;

use crate::twitch_utils::TwitchAPIWrapper;
use crate::_types::twitchtypes::ChatLog;
use crate::_types::clptypes::{UserChatPerformance, MetadataTypes};

use crate::metadata::get_metadata;
use crate::metrics::get_metrics;
use crate::leaderboards::get_leaderboards;

#[allow(dead_code)]
pub struct ChatLogProcessor {
    /*
    Processes the chat logs.

    The class uses the metrics package to extract metrics from the
    chat messages, the metadata package to extract any user-metadata,
    and the leaderboards package to export the metrics / required user
    metadata to the right people
    */
    twitch: TwitchAPIWrapper,
}

impl ChatLogProcessor {
    #[allow(dead_code)]
    pub fn new(twitch: TwitchAPIWrapper) -> Self {
        Self { twitch }
    }

    pub fn __parse_to_log_struct(&self, chat_log_path: String) -> ChatLog {
        let chat_log_str = fs::read_to_string(chat_log_path).unwrap();
        let chat_log: ChatLog = serde_json::from_str(&chat_log_str).unwrap();
        chat_log
    }

    #[allow(dead_code)]
    pub async fn parse_from_log_object(&self, chat_log: ChatLog) -> Vec<UserChatPerformance> {
        let start_time = Instant::now();
        
        info!("Parsing chat log object");
        let mut pre_performance: HashMap<String, UserChatPerformance> = HashMap::new();

        let mut metric_structs = get_metrics();
        let metadata_structs = get_metadata(self.twitch.clone()).await;

        for (sequence_no, comment) in chat_log.comments.iter().enumerate() {
            debug!("Processing comment by user: {} (message: {} of {})", comment.commenter._id, sequence_no, chat_log.comments.len());

            let sequence_no = sequence_no as u32;
            pre_performance.entry(comment.commenter._id.clone()).or_insert(UserChatPerformance {
                id: comment.commenter._id.clone(),
                username: comment.commenter.display_name.clone(),
                avatar: comment.commenter.logo.clone(),
                metrics: metric_structs.iter().map(|m| (m.get_name().to_string(), 0.0)).collect(),
                metadata: metadata_structs.iter().map(|m| (m.get_name().to_string(), m.get_default_value())).collect(),
            });

            let metric_update_map: HashMap<String, HashMap<String, f32>> = metric_structs.iter_mut().map(|m| {
                let metric_name = m.get_name();
                let metric_score = m.get_metric(comment.clone(), sequence_no).clone();
                (metric_name.to_string(), metric_score)
            }).collect();

            let metadata_update_map: HashMap<String, HashMap<String, MetadataTypes>> = metadata_structs.iter().map(|m| {
                let metadata_name = m.get_name();
                let metadata_score = m.get_metadata(comment.clone(), sequence_no);
                (metadata_name.to_string(), metadata_score)
            }).collect();

            debug!("Metric update map: {:?}", metric_update_map);
            debug!("Metadata update map: {:?}", metadata_update_map);

            for (metric_name, update) in metric_update_map.iter() {
                for (user_id, met_value) in update.iter() {
                    // NOTE: the user_id will definitely exist
                    if let Some(user_chat_performance) = pre_performance.get_mut(user_id) {
                        if let Some(metric_value) = user_chat_performance.metrics.get_mut(metric_name) {
                            *metric_value += met_value;
                        }
                    }
                }
            }

            for (metadata_name, update) in metadata_update_map.iter() {
                for (user_id, met_value) in update.iter() {
                    // NOTE: the user_id will definitely exist
                    if let Some(user_chat_performance) = pre_performance.get_mut(user_id) {
                        if let Some(metadata_value) = user_chat_performance.metadata.get_mut(metadata_name) {
                            *metadata_value = met_value.clone();
                        }
                    }
                }
            }
        }

        // Flush final metric updates
        let metric_update_map: HashMap<String, HashMap<String, f32>> = metric_structs.iter().map(|m| {
            let metric_name = m.get_name();
            let metric_score = m.finish();
            (metric_name.to_string(), metric_score)
        }).collect();

        debug!("Final metric update map: {:?}", metric_update_map);

        for (metric_name, update) in metric_update_map.iter() {
            for (user_id, met_value) in update.iter() {
                // NOTE: the user_id will definitely exist
                if let Some(user_chat_performance) = pre_performance.get_mut(user_id) {
                    if let Some(metric_value) = user_chat_performance.metrics.get_mut(metric_name) {
                        *metric_value += met_value;
                    }
                }
            }
        }

        let elapsed = start_time.elapsed();
        info!("Chat log processing took: {}ms", elapsed.as_millis());

        pre_performance.values().cloned().collect()
    }

    #[allow(dead_code)]
    async fn parse(&self, chat_log_path: String) -> Vec<UserChatPerformance> {
        let chat_log = self.__parse_to_log_struct(chat_log_path);
        self.parse_from_log_object(chat_log).await
    }

    #[allow(dead_code)]
    pub fn export_to_leaderboards(performances: Vec<UserChatPerformance>) {
        let mut leaderboards = get_leaderboards();
        for leaderboard in leaderboards.iter_mut() {
            for performance in performances.iter() {
                leaderboard.update_leaderboard(performance.clone());
            }
            leaderboard.save();
        }
    }
}