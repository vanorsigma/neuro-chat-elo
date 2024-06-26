/* 
A function to backfill given video IDs
*/

use log::info;

use crate::twitch_utils::TwitchAPIWrapper;
use crate::twitchdownloaderproxy::TwitchChatDownloader;
use crate::chatlogprocessor::ChatLogProcessor;

const VIDEO_IDS: [&str; 6] = [
    "2170316549",
    "2171991671",
    "2172878349",
    "2176205867",
    "2175349344",
    "2178862405"
];

#[allow(dead_code)]
pub async fn backfill() {
    let twitch = TwitchAPIWrapper::new().await.unwrap();
    let mut downloader = TwitchChatDownloader::new();

    for video_id in VIDEO_IDS.iter() {
        info!("Backfilling for video ID: {}", video_id);
        let chat_log = downloader.download_chat(video_id).await.unwrap();

        let processor = ChatLogProcessor::new(twitch.clone());
        let user_performances = processor.parse_from_log_object(chat_log).await;

        ChatLogProcessor::export_to_leaderboards(user_performances);
    }
}