/*
A function to backfill given video IDs
*/

use std::env;
use std::sync::Arc;

use elo::_types::clptypes::Message;
use log::info;
use twitch_utils::seventvclient::SevenTVClient;

use crate::chatlogprocessor::ChatLogProcessor;
use crate::twitchdownloaderproxy::TwitchChatDownloader;
use crate::{adventuresdownloaderproxy, discorddownloaderproxy, CHANNEL_ID};
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
            .expect("Could not download chat log: {e:?}")
            .comments
            .into_iter()
            .map(Message::Twitch);

        let discord_messages = match env::var("CHAT_DISCORD_TOKEN") {
            Ok(token) => {
                let (start_time, end_time) = twitch.get_vod_times(video_id.to_string()).await;
                discorddownloaderproxy::DiscordChatDownloader::new()
                    .download_chat(
                        start_time.into(),
                        end_time.into(),
                        CHANNEL_ID,
                        token.as_str(),
                    )
                    .await
                    .expect("Failed to download Discord chat")
                    .messages
            }
            _ => {
                vec![]
            }
        }
        .into_iter()
        .map(Message::Discord);

        let user_performances = ChatLogProcessor::new(&twitch, seventv_client.clone())
            .await
            .process_from_messages(chat_log.chain(discord_messages))
            .await;

        ChatLogProcessor::export_to_leaderboards(user_performances).await;
    }

    if let Ok(token) = std::env::var("CHAT_DISCORD_TOKEN") {
        let adventure_ranks = adventuresdownloaderproxy::AdventuresDownloaderProxy::new(token)
            .get_ranks()
            .await
            .unwrap();

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
            .await;
    }
}
