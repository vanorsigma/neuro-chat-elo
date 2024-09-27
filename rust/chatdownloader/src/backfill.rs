/*
A function to backfill given video IDs
*/

use core::error;
use std::env;
use std::sync::Arc;

use discord_utils::DiscordClient;
use elo::_types::clptypes::Message;
use log::info;
use twitch_utils::seventvclient::SevenTVClient;

use crate::chatlogprocessor::ChatLogProcessor;
use crate::discorddownloaderproxy::DiscordChatDownloader;
use crate::twitchdownloaderproxy::TwitchChatDownloader;
use crate::{adventuresdownloaderproxy, discorddownloaderproxy, CHANNEL_ID};
use twitch_utils::TwitchAPIWrapper;

pub async fn backfill() {
    let discord_token = env::var("CHAT_DISCORD_TOKEN")
        .inspect_err(|err| {
            log::warn!("No Discord token. continuing");
        })
        .unwrap_or("".to_string());
    let discord = Arc::new(DiscordClient::new(discord_token.to_string()));

    if let Ok(cache_path) = env::var("DISCORD_USER_CACHE") {
        discord
            .preload_cache(&cache_path)
            .await
            .expect("cache path configured, but cannot preload");
    } else {
        log::warn!("No discord user cache, not preloading discord client")
    }

    let twitch = Arc::new(TwitchAPIWrapper::new().await.unwrap());
    let seventv_client = Arc::new(SevenTVClient::new().await);
    let video_ids = twitch
        .get_latest_vod_ids(elo::_constants::VED_CH_ID.to_string(), 5)
        .await;
    let mut downloader = TwitchChatDownloader::new();
    let mut additional_messages = vec![];

    for video_id in video_ids.iter() {
        info!("Backfilling for video ID: {}", video_id);
        // let chat_log = downloader.download_chat(video_id).await.unwrap();
        let chat_log = downloader
            .download_chat(video_id)
            .await
            .expect("Could not download chat log: {e:?}")
            .comments;

        for comment in &chat_log {
            twitch.set_user_cache(comment.commenter.clone()).await;
        }

        let chat_log = chat_log.into_iter().map(Message::Twitch);

        let discord_messages = futures::future::join_all(
            match discord_token.as_str() {
                "" => {
                    vec![]
                }
                token => {
                    let (start_time, end_time) = twitch.get_vod_times(video_id.to_string()).await;
                    discorddownloaderproxy::DiscordChatDownloader::new()
                        .download_chat(start_time.into(), end_time.into(), CHANNEL_ID, token)
                        .await
                        .expect("Failed to download Discord chat")
                        .messages
                }
            }
            .into_iter()
            .map(|message| async {
                discord.set_author_cache(message.author.clone()).await;
                message
            })
            .map(|message_async| async { Message::Discord(message_async.await) }),
        )
        .await;

        let user_performances =
            ChatLogProcessor::new(twitch.clone(), seventv_client.clone(), discord.clone())
                .await
                // .process_from_messages(chat_log)
                .process_from_messages(chat_log.chain(discord_messages))
                .await;

        ChatLogProcessor::export_to_leaderboards(
            user_performances,
            twitch.clone(),
            discord.clone(),
        )
        .await;
    }

    let clp = ChatLogProcessor::new(twitch.clone(), seventv_client.clone(), discord.clone()).await;

    if let Ok(_) = std::env::var("CHAT_DISCORD_TOKEN") {
        let adventure_ranks =
            adventuresdownloaderproxy::AdventuresDownloaderProxy::new(discord.clone())
                .get_ranks()
                .await
                .unwrap();

        additional_messages.extend(
            adventure_ranks
                .get("The Farm")
                .unwrap()
                .into_iter()
                .cloned()
                .map(|item| Message::Adventures(item)),
        );
    }

    if std::fs::exists("pxls.json").unwrap_or(false) {
        info!("Found pxls.json, will export pxls leaderboard");
        additional_messages.extend(
            pxls_utils::PxlsJsonReader::read_pxls_from_json_path("pxls.json")
                .expect("should have pxls json")
                .into_iter()
                .map(Message::Pxls),
        );
    }

    ChatLogProcessor::export_to_leaderboards(
        clp.process_from_messages(additional_messages.into_iter())
            .await,
        twitch,
        discord,
    )
    .await;
}
