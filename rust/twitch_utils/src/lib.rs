use std::collections::HashMap;

use dotenv::dotenv;
use log::debug;
use serde::{Deserialize, Serialize};
use twitch_api::helix::chat::{BadgeSet, ChatBadge, GetChannelChatBadgesRequest, GetGlobalChatBadgesRequest};
use twitch_api::helix::videos::GetVideosRequest;
use twitch_api::twitch_oauth2::{AppAccessToken, ClientId, ClientSecret};
use twitch_api::HelixClient;

pub const USER_AGENT: &str = concat!(
    "neuro-chat-elo/0.1 ",
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (https://vanorsigma.github.io/neuro-chat-elo)"
);

pub mod twitchtypes;

#[derive(Clone)]
pub struct TwitchAPIWrapper {
    pub twitch: HelixClient<'static, reqwest::Client>,
    pub token: AppAccessToken,
}

impl TwitchAPIWrapper {
    pub async fn new() -> Result<Self, reqwest::Error> {
        dotenv().ok();
        let client_id: ClientId = std::env::var("TWITCH_APPID")
            .map(ClientId::new)
            .expect("TWITCH_APPID must be set");

        let client_secret: ClientSecret = std::env::var("TWITCH_APPSECRET")
            .map(ClientSecret::new)
            .expect("TWITCH_APPSECRET must be set");

        debug!("Creating HTTP Client for TwitchAPIWrapper");
        let http_client = reqwest::ClientBuilder::new()
            .user_agent(USER_AGENT)
            .build()
            .expect("Failed to create HTTP Client");

        let twitch: HelixClient<'static, reqwest::Client> =
            twitch_api::HelixClient::with_client(http_client.clone());

        let token =
            AppAccessToken::get_app_access_token(&http_client, client_id, client_secret, vec![])
                .await
                .unwrap();

        Ok(Self { twitch, token })
    }

    pub async fn get_latest_vod_id(&self, ch_id: String) -> String {
        let request = GetVideosRequest::user_id(ch_id.clone());
        let response = self.twitch.req_get(request, &self.token);
        response.await.unwrap().data[0].id.clone().to_string()
    }

    pub async fn get_badges(
        &self,
        ch_id: String,
    ) -> Result<HashMap<String, HashMap<String, ChatBadge>>, reqwest::Error> {
        let request = GetChannelChatBadgesRequest::broadcaster_id(ch_id.clone());
        let response = self.twitch.req_get(request, &self.token);
        let channel_badges = response.await.unwrap().data;

        let request = GetGlobalChatBadgesRequest::new();
        let response = self.twitch.req_get(request, &self.token);
        let global_badges = response.await.unwrap().data;

        let all_badges = [global_badges, channel_badges].concat();

        let mut badge_sets: HashMap<String, HashMap<String, ChatBadge>> = HashMap::new();

        for badge_set in all_badges {
            let mut badges: HashMap<String, ChatBadge> = HashMap::new();

            for badge in badge_set.versions {
                badges.insert(badge.id.to_string().clone(), badge);
            }

            badge_sets.insert(badge_set.set_id.to_string().clone(), badges);
        }

        Ok(badge_sets)
    }
}
