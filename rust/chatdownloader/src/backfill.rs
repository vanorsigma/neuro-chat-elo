/*
A function to backfill given video IDs
*/

use log::info;

use crate::chatlogprocessor::ChatLogProcessor;
use crate::twitchdownloaderproxy::TwitchChatDownloader;
use twitch_utils::TwitchAPIWrapper;

const VIDEO_IDS: [&str; 12] = [
    "2170316549",
    "2171991671",
    "2172878349",
    "2176205867",
    "2175349344",
    "2178862405",
    "2188296968",
    "2187465183",
    "2182332760",
    "2181468979",
    "2180615386",
    "2179780834",
];

pub async fn backfill() {
    let twitch = TwitchAPIWrapper::new().await.unwrap();
    let mut downloader = TwitchChatDownloader::new();

    for video_id in VIDEO_IDS.iter() {
        info!("Backfilling for video ID: {}", video_id);
        // let chat_log = downloader.download_chat(video_id).await.unwrap();
        let chat_log = downloader
            .download_chat(video_id)
            .await
            .expect("Could not download chat log: {e:?}");

        let user_performances = ChatLogProcessor::new(&twitch)
            .await
            .process_from_log_object(chat_log)
            .await;

        ChatLogProcessor::export_to_leaderboards(user_performances).await;
    }
}
