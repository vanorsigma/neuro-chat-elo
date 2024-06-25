mod _types;
mod _constants;
mod metrics;
mod metadata;
mod leaderboards;
mod chatlogprocessor;
mod twitchdownloaderproxy;
mod twitch_utils;

use log::info;
use env_logger::Env;

#[tokio::main]
async fn main() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    info!("Authenticating with Twitch...");
    let twitch = twitch_utils::TwitchAPIWrapper::new().await.unwrap();
    let vod_id = twitch.get_latest_vod_id(_constants::VED_CH_ID.to_string()).await;

    info!("Script triggered, pulling logs for VOD ID: {}...", vod_id);

    let mut downloader = twitchdownloaderproxy::TwitchChatDownloader::new();
    let chat_log = downloader.download_chat(&vod_id).await.unwrap();
    
    let processor = chatlogprocessor::ChatLogProcessor::new(twitch);
    // let chat_log = processor.__parse_to_log_struct("chat.json".to_string());
    let user_performances = processor.parse_from_log_object(chat_log).await;
    chatlogprocessor::ChatLogProcessor::export_to_leaderboards(user_performances);
}
