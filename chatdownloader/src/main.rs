mod _constants;
mod _types;
mod backfill;
mod chatlogprocessor;
mod leaderboards;
mod metadata;
mod metrics;
mod twitch_utils;
mod twitchdownloaderproxy;

use env_logger::Env;
use log::info;
use std::{env, process::exit};

#[tokio::main]
async fn main() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    if env::var("BACKFILL").as_deref() == Ok("1") {
        backfill::backfill().await;
        exit(0);
    }

    info!("Authenticating with Twitch...");

    let twitch = twitch_utils::TwitchAPIWrapper::new().await.unwrap();
    let vod_id = twitch
        .get_latest_vod_id(_constants::VED_CH_ID.to_string())
        .await;

    info!("Script triggered, pulling logs for VOD ID: {}...", vod_id);

    let mut downloader = twitchdownloaderproxy::TwitchChatDownloader::new();
    let chat_log = match downloader.download_chat(&vod_id).await {
        Ok(chat_log) => chat_log,
        Err(e) => panic!("Failed to download chat: {e:?}"),
    };

    let processor = chatlogprocessor::ChatLogProcessor::new(&twitch);
    // let chat_log = processor.__parse_to_log_struct("chat.json".to_string());
    let user_performances = processor.parse_from_log_object(chat_log).await;
    chatlogprocessor::ChatLogProcessor::export_to_leaderboards(user_performances).await;
}
