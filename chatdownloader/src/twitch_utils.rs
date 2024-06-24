use twitch_api::twitch_oauth2::{
    tokens::errors::AppAccessTokenError, AppAccessToken, TwitchToken,
};
use twitch_api::{helix::channels::GetChannelInformationRequest, TwitchClient};
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn get_auth_twitch() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load .env file if exists
    let app_id = env::var("TWITCH_APPID")?;
    let app_secret = env::var("TWITCH_APPSECRET")?;

    // Create the HelixClient, which is used to make requests to the Twitch API
    let client: HelixClient<reqwest::Client> = HelixClient::default();
    // Create a UserToken, which is used to authenticate requests
    let token = UserToken::from_token(&client, AccessToken::from("mytoken")).await?;

    println!(
        "Channel: {:?}",
        client.get_channel_from_login("twitchdev", &token).await?
    );

    Ok(())
}

async fn get_latest_vod(client: &TwitchClient, ch_id: &str) -> Result<String, Box<dyn std::error::Error>> {
    let videos = client.get_videos()
        .user_id(ch_id)
        .first(1)
        .execute()
        .await?;

    let latest_vod_id = videos.data.first()
        .ok_or("No VODs available for the given channel")?
        .id.clone();

    println!("Latest VOD ID: {}", latest_vod_id);
    Ok(latest_vod_id)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let twitch_client = get_auth_twitch().await?;
    let latest_vod_id = get_latest_vod(&twitch_client, "channel_id").await?;
    println!("Latest VOD ID is: {}", latest_vod_id);

    Ok(())
}
