mod adventuresdownloaderproxy;
mod backfill;
mod bilidownloaderproxy;
mod chatlogprocessor;
mod discorddownloaderproxy;
mod github;
mod twitchdownloaderproxy;

use discord_utils::DiscordClient;
use elo::_types::clptypes::Message;
use env_logger::Env;
use log::info;
use std::{env, path::Path, process::exit, sync::Arc};
use twitch_utils::{seventvclient::SevenTVClient, TwitchAPIWrapper};

const CHANNEL_ID: &str = "1067638175478071307";

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    if env::var("BACKFILL").as_deref() == Ok("1") {
        backfill::backfill().await;
        exit(0);
    }

    let token = std::env::var("CHAT_DISCORD_TOKEN").unwrap();

    info!("Authenticating with Twitch...");

    let twitch = Arc::new(TwitchAPIWrapper::new().await.unwrap());
    let vod_id = twitch
        .get_latest_vod_ids(elo::_constants::VED_CH_ID.to_string(), 1)
        .await[0]
        .clone();

    info!("Script triggered, pulling logs for VOD ID: {}...", vod_id);

    let discord = Arc::new(DiscordClient::new(token));

    if let Ok(cache_path) = env::var("DISCORD_USER_CACHE") {
        discord
            .preload_cache(&cache_path)
            .await
            .expect("cache path configured, but cannot preload");
    } else {
        log::warn!("No discord user cache, not preloading discord client")
    }

    let mut downloader = twitchdownloaderproxy::TwitchChatDownloader::new();

    let adventure_ranks =
        adventuresdownloaderproxy::AdventuresDownloaderProxy::new(discord.clone())
            .get_ranks()
            .await
            .unwrap();

    let adventures_farm = adventure_ranks
        .get("The Farm")
        .unwrap()
        .into_iter()
        .cloned()
        .map(|item| Message::Adventures(item));

    let chat_log = downloader
        .download_chat(&vod_id)
        .await
        .expect("Failed to download chat")
        .comments
        .into_iter()
        .map(Message::Twitch);

    let discord_messages = futures::future::join_all(
        match env::var("CHAT_DISCORD_TOKEN") {
            Ok(token) => {
                let (start_time, end_time) = twitch.get_vod_times(vod_id).await;
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
        .map(|message| async {
            discord.set_author_cache(message.author.clone()).await;
            message
        })
        .map(|x| async { Message::Discord(x.await) }),
    )
    .await;

    let pxls_casual = pxls_utils::PxlsJsonReader::read_pxls_from_json_path("pxls.json")
        .expect("should have pxls json")
        .into_iter()
        .map(Message::Pxls);

    let bilibili_messages = bilidownloaderproxy::BiliChatDownloader::new()
        .from_path(Path::new("./output_fixed_fixed.json"))
        .into_iter()
        .map(|m| Message::Bilibili(m));

    let seventv_client = Arc::new(SevenTVClient::new().await);

    let processor =
        chatlogprocessor::ChatLogProcessor::new(twitch.clone(), seventv_client, discord.clone())
            .await;
    // let chat_log = processor.__parse_to_log_struct("chat.json".to_string());
    let user_performances = processor
        .process_from_messages(
            chat_log
                .chain(discord_messages)
                .chain(bilibili_messages)
                .chain(adventures_farm)
                .chain(pxls_casual),
        )
        .await;
    chatlogprocessor::ChatLogProcessor::export_to_leaderboards(
        user_performances,
        twitch,
        discord.clone(),
    )
    .await;

    if let Ok(cache_path) = env::var("DISCORD_USER_CACHE") {
        discord
            .dump_cache(&cache_path)
            .await
            .expect("cache path configured, dumped cache");
    } else {
        log::warn!("No discord user cache, not dumping discord cache")
    }
}
