mod _types;
mod _constants;
mod metrics;
mod leaderboards;
mod chatlogprocessor;
mod twitchdownloaderproxy;
mod twitch_utils;

use log::info;

#[tokio::main]
async fn main() {
    info!("Authenticating with Twitch...");

    let twitch = twitch_utils::Twitch::new().await.unwrap();
    let vod_id = twitch.get_latest_vod_id(_constants::VED_CH_ID.to_string()).await;

    info!("Script triggered, pulling logs for VOD ID: {}...", vod_id);

    let mut downloader = twitchdownloaderproxy::TwitchChatDownloader::new();
    let chat_log = downloader.download_chat(&vod_id).await.unwrap();
    println!("{:?}", chat_log);
}
