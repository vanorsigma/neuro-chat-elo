/*
A function to backfill given video IDs
*/

use std::env;
use std::sync::Arc;

use elo::_types::clptypes::Message;
use log::info;
use twitch_utils::seventvclient::SevenTVClient;

use crate::chatlogprocessor::ChatLogProcessor;
use crate::discorddownloaderproxy;
use crate::twitchdownloaderproxy::TwitchChatDownloader;
use twitch_utils::TwitchAPIWrapper;

const CHANNEL_ID: &str = "1067638175478071307";
const VIDEO_IDS: &[&str] = &[
    "2236352117",
    "2235486105",
    "2234631158",
    "2230251136",
    "2229374664",
    "2238030872",
];

pub async fn backfill() {
    let twitch = TwitchAPIWrapper::new().await.unwrap();
    let seventv_client = Arc::new(SevenTVClient::new().await);
    let mut downloader = TwitchChatDownloader::new();

    for video_id in VIDEO_IDS.iter() {
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
            .map(|m| Message::Discord(m));

        let user_performances = ChatLogProcessor::new(&twitch, seventv_client.clone())
            .await
            .process_from_messages(chat_log.chain(discord_messages))
            .await;

        ChatLogProcessor::export_to_leaderboards(user_performances).await;
    }
}
