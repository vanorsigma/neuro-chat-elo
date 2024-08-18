use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};
use log::debug;
use twitch_api::helix::chat::{ChatBadge, GetChannelChatBadgesRequest, GetGlobalChatBadgesRequest};
use twitch_api::helix::videos::GetVideosRequest;
use twitch_api::twitch_oauth2::{AppAccessToken, ClientId, ClientSecret};
use twitch_api::HelixClient;

pub mod seventvclient;
pub mod seventvtypes;
pub mod twitchtypes;

pub const USER_AGENT: &str = concat!(
    "neuro-chat-elo/0.1 ",
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (https://vanorsigma.github.io/neuro-chat-elo)"
);

/// Twitch's duration timestamp can't be parsed by iso8601 parsers that I can find
fn parse_time(duration_str: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let re = regex::Regex::new(r"(?:(\d+)h)?(?:(\d+)m)?(?:(\d+)s)?").unwrap();

    let captures = re
        .captures(duration_str)
        .expect("should pass even with empty string");

    let hours = captures
        .get(1)
        .map_or(Ok(0), |m| m.as_str().parse::<u32>())?;
    let minutes = captures
        .get(2)
        .map_or(Ok(0), |m| m.as_str().parse::<u32>())?;
    let seconds = captures
        .get(3)
        .map_or(Ok(0), |m| m.as_str().parse::<u32>())?;

    Ok(hours * 3600 + minutes * 60 + seconds)
}

#[derive(Clone)]
pub struct TwitchAPIWrapper {
    pub twitch: HelixClient<'static, reqwest::Client>,
    pub token: AppAccessToken,
}

impl TwitchAPIWrapper {
    pub async fn new() -> Result<Self, reqwest::Error> {
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

    pub async fn get_latest_vods(&self, ch_id: String, num: usize) -> Vec<String> {
        let request = GetVideosRequest::user_id(ch_id.clone());
        let response = self.twitch.req_get(request, &self.token);
        response
            .await
            .unwrap()
            .data
            .iter()
            .take(num)
            .map(|v| v.id.clone().to_string())
            .collect()
    }

    /// Returns a tuple (start timestamp and end timestamp) of the VOD
    pub async fn get_vod_times(
        &self,
        vod_id: String,
    ) -> (DateTime<FixedOffset>, DateTime<FixedOffset>) {
        let vod_ids = [(&vod_id).into()];
        let vod_info = self
            .twitch
            .req_get(GetVideosRequest::ids(&vod_ids), &self.token)
            .await
            .unwrap();

        let start_timestamp =
            chrono::DateTime::parse_from_rfc3339(vod_info.data[0].created_at.as_str())
                .expect("can interpret datetime");

        let end_timestamp = start_timestamp
            .checked_add_signed(
                chrono::TimeDelta::new(
                    parse_time(vod_info.data[0].duration.as_str()).expect("parsable duration")
                        as i64,
                    0,
                )
                .expect("can convert to timedelta"),
            )
            .expect("can get end duration");

        (start_timestamp, end_timestamp)
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
