use std::{env, fs, collections::HashMap};
use std::time::Instant;
use log::{debug, LevelFilter};
use serde::{Deserialize, Serialize};
use serde_json;

use twitch_utils::Twitch;
use _types::{ChatLog, Comment, UserChatPerformance};

#[derive(Debug)]
struct ChatLogProcessor {
    twitch: Twitch,
}

impl ChatLogProcessor {
    fn new(twitch: Twitch) -> Self {
        Self { twitch }
    }

    fn parse_from_log_object(&self, chat_log: ChatLog) -> Vec<UserChatPerformance> {
        let start_time = Instant::now();
        let mut pre_performance: HashMap<String, UserChatPerformance> = HashMap::new();
        let metric_instances: Vec<Box<dyn metrics::Metric>> = metrics::EXPORTED_METRICS.iter().map(|m| m()).collect();
        let metadata_instances: Vec<Box<dyn metadata::Metadata>> = metadata::EXPORTED_METADATA.iter().map(|m| m(&self.twitch)).collect();

        for (seq_no, comment) in chat_log.comments.iter().enumerate() {
            debug!("Processing comment by {} (message {} of {})",
                   comment.commenter.display_name, seq_no + 1, chat_log.comments.len());

            pre_performance.entry(comment.commenter.id)
                .or_insert_with(|| UserChatPerformance::new(&comment.commenter, &metric_instances, &metadata_instances));

            let metric_update_arr: HashMap<String, _> = metric_instances.iter().map(|metric| {
                let metric_name = metric.get_name();
                let metric_value = debug_timing(metric_name, || metric.get_metric(comment, seq_no));
                (metric_name.to_string(), metric_value)
            }).collect();

            let metadata_update_arr: HashMap<String, _> = metadata_instances.iter().map(|metadata| {
                let metadata_name = metadata.get_name();
                let metadata_value = debug_timing(metadata_name, || metadata.get_metadata(comment, seq_no));
                (metadata_name.to_string(), metadata_value)
            }).collect();

            debug!("Metric Update Array: {:?}", metric_update_arr);
            debug!("Metadata Update Array: {:?}", metadata_update_arr);

            // Update performance records
            for (k, v) in metric_update_arr {
                pre_performance.get_mut(&comment.commenter.id).unwrap().metrics.insert(k, v);
            }
            for (k, v) in metadata_update_arr {
                pre_performance.get_mut(&comment.commenter.id).unwrap().metadata.insert(k, v);
            }
        }

        // Finalize metrics
        let metric_final_update: HashMap<_, _> = metric_instances.iter().map(|metric| {
            let metric_name = metric.get_name();
            let metric_value = debug_timing(metric_name, || metric.finish());
            (metric_name.to_string(), metric_value)
        }).collect();

        debug!("Finalize - Metric Update Array: {:?}", metric_final_update);

        for (k, v) in metric_final_update {
            for user_id in pre_performance.keys() {
                *pre_performance.get_mut(user_id).unwrap().metrics.get_mut(&k).unwrap() += v;
            }
        }

        let process_duration = start_time.elapsed();
        debug!("Chat log processor took {:?} seconds to process the logs", process_duration);

        pre_performance.into_values().collect()
    }

    fn parse(&self, chat_log_path: &str) -> Vec<types::UserChatPerformance> {
        let chat_log_data = fs::read_to_string(chat_log_path).expect("Failed to read chat log file");
        let chat_log: types::ChatLog = serde_json::from_str(&chat_log_data).expect("Failed to parse chat log");
        self.parse_from_log_object(chat_log)
    }

    fn export_to_leaderboards(performances: Vec<types::UserChatPerformance>) {
        let leaderboards: Vec<Box<dyn leaderboards::Leaderboard>> = leaderboards::EXPORTED_LEADERBOARDS.iter().map(|l| l()).collect();
        for leaderboard in leaderboards {
            for performance in &performances {
                leaderboard.update_leaderboard(performance);
            }
            leaderboard.save();
        }
    }
}

fn main() {
    env_logger::builder().filter_level(LevelFilter::Debug).init();

    let twitch = twitch_api::Twitch::new();
    let clp = ChatLogProcessor::new(twitch);
    let result = clp.parse("src/result.json");
    ChatLogProcessor::export_to_leaderboards
