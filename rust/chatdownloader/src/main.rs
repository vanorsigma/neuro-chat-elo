mod backfill;
mod chatlogprocessor;
mod twitchdownloaderproxy;

use env_logger::Env;
use log::info;
use std::{env, process::exit};
use twitch_utils::TwitchAPIWrapper;

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

    info!("Authenticating with Twitch...");

    let twitch = TwitchAPIWrapper::new().await.unwrap();
    let vod_id = twitch
        .get_latest_vod_id(elo::_constants::VED_CH_ID.to_string())
        .await;

    info!("Script triggered, pulling logs for VOD ID: {}...", vod_id);

    let mut downloader = twitchdownloaderproxy::TwitchChatDownloader::new();
    let chat_log = downloader
        .download_chat(&vod_id)
        .await
        .expect("Failed to download chat: {e:?}");

    let processor = chatlogprocessor::ChatLogProcessor::new(&twitch).await;
    // let chat_log = processor.__parse_to_log_struct("chat.json".to_string());
    let user_performances = processor.process_from_log_object(chat_log).await;
    chatlogprocessor::ChatLogProcessor::export_to_leaderboards(user_performances).await;
}
