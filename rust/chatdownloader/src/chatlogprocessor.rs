use elo::MessageProcessor;
use elo::_types::clptypes::{Message, UserChatPerformance};
use elo::leaderboards::{LeaderboardProcessor, LeaderboardProcessorBuilder};
use log::{debug, info};
use std::fs;
use std::time::Instant;
use twitch_utils::TwitchAPIWrapper;

use twitch_utils::twitchtypes::ChatLog;

pub struct ChatLogProcessor {
    /*
    Processes the chat logs.

    The class uses the metrics package to extract metrics from the
    chat messages, the metadata package to extract any user-metadata,
    and the leaderboards package to export the metrics / required user
    metadata to the right people
    */
    message_processor: MessageProcessor,
}

impl ChatLogProcessor {
    pub async fn new(twitch: &TwitchAPIWrapper) -> Self {
        let message_processor = MessageProcessor::new(twitch).await;

        Self { message_processor }
    }

    pub(crate) fn parse_to_log_struct(&self, chat_log_path: String) -> ChatLog {
        let chat_log_str = fs::read_to_string(chat_log_path).unwrap();
        serde_json::from_str(&chat_log_str).unwrap()
    }

    pub async fn process_from_messages<Iter: Iterator<Item = Message>>(
        self,
        messages: Iter,
    ) -> Vec<UserChatPerformance> {
        let start_time = Instant::now();
        debug!("Starting chat log processing");

        for message in messages {
            self.message_processor
                .process_message(message.clone())
                .await;
        }

        let performances = self.message_processor.finish().await;

        info!("Chat log processing took: {:#?}", start_time.elapsed());
        performances.into_values().collect()
    }

    pub async fn process_from_log_object(self, chat_log: ChatLog) -> Vec<UserChatPerformance> {
        self.process_from_messages(
            chat_log
                .comments
                .into_iter()
                .map(|comment| Message::from(comment)),
        )
        .await
    }

    #[allow(dead_code)]
    async fn process(self, chat_log_path: String) -> Vec<UserChatPerformance> {
        let chat_log = self.parse_to_log_struct(chat_log_path);
        self.process_from_log_object(chat_log).await
    }

    /// A function to export the user performances to the leaderboards and save them
    pub async fn export_to_leaderboards(performances: Vec<UserChatPerformance>) {
        let leaderboard_processor = LeaderboardProcessorBuilder::all_leaderboards().spawn();
        for performance in performances {
            leaderboard_processor.send_performance(performance);
        }
        leaderboard_processor.finish().await;
    }
}
