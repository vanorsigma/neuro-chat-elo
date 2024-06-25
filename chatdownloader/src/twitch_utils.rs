use std::collections::HashMap;

use reqwest;
use dotenv::dotenv;
use twitch_api::HelixClient;
use twitch_api::twitch_oauth2::{ClientId, ClientSecret, AppAccessToken};
use twitch_api::helix::videos::GetVideosRequest;
use twitch_api::helix::chat::{GetChannelChatBadgesRequest, GetGlobalChatBadgesRequest};

use crate::_types::clptypes::BadgeInformation;

const USER_AGENT: &str = concat!(
    "neuro-chat-elo/0.1 ",
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (https://vanorsigma.github.io/neuro-chat-elo)"
);

#[allow(dead_code)]
#[derive(Clone)]
pub struct TwitchAPIWrapper {
    pub twitch: HelixClient<'static, reqwest::Client>,
    pub token: AppAccessToken
}

impl TwitchAPIWrapper {
    #[allow(dead_code)]
    pub async fn new() -> Result<Self, reqwest::Error>{
        dotenv().ok();
        let client_id: ClientId = std::env::var("TWITCH_APPID")
            .map(ClientId::new)
            .expect("TWITCH_APPID must be set");

        let client_secret: ClientSecret = std::env::var("TWITCH_APPSECRET")
            .map(ClientSecret::new)
            .expect("TWITCH_APPSECRET must be set");

        let http_client = crate::twitch_utils::reqwest::ClientBuilder::new()
            .user_agent(USER_AGENT)
            .build()?;

        let twitch: HelixClient<'static, reqwest::Client> = twitch_api::HelixClient::with_client(http_client.clone());

        let token = AppAccessToken::get_app_access_token(
            &http_client,
            client_id,
            client_secret,
            vec![]
        ).await.unwrap();

        Ok(Self {
            twitch,
            token
        })
    }

    #[allow(dead_code)]
    pub async fn get_latest_vod_id(&self, ch_id: String) -> String {
        let request = GetVideosRequest::user_id(ch_id.clone());
        let response = self.twitch.req_get(request, &self.token);
        response.await.unwrap().data[0].id.clone().to_string()
    }

    #[allow(dead_code)]
    pub async fn get_badges(&self, ch_id: String) -> Result<HashMap<String, HashMap<String, BadgeInformation>>, reqwest::Error> {
        let request = GetChannelChatBadgesRequest::broadcaster_id(ch_id.clone());
        let response = self.twitch.req_get(request, &self.token);
        let channel_badges = response.await.unwrap().data;

        let request = GetGlobalChatBadgesRequest::new();
        let response = self.twitch.req_get(request, &self.token);
        let global_badges = response.await.unwrap().data;

        let all_badges = [channel_badges, global_badges].concat();

        let mut badge_sets: HashMap<String, HashMap<String, BadgeInformation>> = HashMap::new();

        for badge_set in all_badges {
            let mut badges: HashMap<String, BadgeInformation> = HashMap::new();

            for badge in badge_set.versions {

                let badge_info = BadgeInformation {
                    description: badge_set.set_id.to_string().clone(),
                    image_url: badge.image_url_4x.clone()
                };
                badges.insert(badge.id.to_string().clone(), badge_info);
            }
            
            badge_sets.insert(badge_set.set_id.to_string().clone(), badges);
        }

        Ok(badge_sets)
    }
}