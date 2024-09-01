/*
A function to backfill given video IDs
*/

use std::sync::Arc;

use elo::_types::clptypes::Message;
use log::info;
use twitch_utils::seventvclient::SevenTVClient;

use crate::adventuresdownloaderproxy;
use crate::chatlogprocessor::ChatLogProcessor;
use crate::twitchdownloaderproxy::TwitchChatDownloader;
use twitch_utils::TwitchAPIWrapper;

pub async fn backfill() {
    let twitch = TwitchAPIWrapper::new().await.unwrap();
    let seventv_client = Arc::new(SevenTVClient::new().await);
    let video_ids = twitch
        .get_latest_vod_ids(elo::_constants::VED_CH_ID.to_string(), 5)
        .await;
    let mut downloader = TwitchChatDownloader::new();

    for video_id in video_ids.iter() {
        info!("Backfilling for video ID: {}", video_id);
        // let chat_log = downloader.download_chat(video_id).await.unwrap();
        let chat_log = downloader
            .download_chat(video_id)
            .await
            .expect("Could not download chat log: {e:?}");

        let user_performances = ChatLogProcessor::new(&twitch, seventv_client.clone())
            .await
            .process_from_log_object(chat_log)
            .await;

        ChatLogProcessor::export_to_leaderboards(user_performances).await;
    }

    if let Ok(token) = std::env::var("CHAT_DISCORD_TOKEN") {
        let adventure_ranks = adventuresdownloaderproxy::AdventuresDownloaderProxy::new(token)
            .get_ranks()
            .await
            .unwrap();

        ChatLogProcessor::export_to_leaderboards(
            ChatLogProcessor::new(&twitch, seventv_client.clone())
                .await
                .process_from_messages(
                    adventure_ranks
                        .get("The Farm")
                        .unwrap()
                        .into_iter()
                        .cloned()
                        .map(|item| Message::Adventures(item)),
                )
                .await,
        )
        .await;
    }
}
