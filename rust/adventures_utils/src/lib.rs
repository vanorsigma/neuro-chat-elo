use serde::{self, Deserialize, Serialize};

const BADGE_URL: &str = "https://neuroranks.chat/neuro-chat-elo/{ai}-{collab}.png";

#[derive(Serialize, Default, Debug)]
pub enum AdventuresMetadataType {
    #[default]
    #[serde(rename = "get_metadata")]
    GetMetadata,
}

#[derive(Serialize, Default, Debug)]
pub enum AdventuresGetLeaderboardType {
    #[default]
    #[serde(rename = "get_leaderboard")]
    GetLeaderboard,
}

#[derive(Serialize, Default, Debug)]
pub struct AdventuresMetadataRequest {
    #[serde(rename = "type")]
    ty: AdventuresMetadataType,
}

#[derive(Deserialize, Debug)]
pub struct AdventuresMetadataVersion {
    #[serde(rename = "versionNumber")]
    pub version_number: String,
    pub maps: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct AdventuresMetadataResponse {
    pub versions: Vec<AdventuresMetadataVersion>,
}

#[derive(Serialize, Debug)]
pub struct AdventuresGetLeaderboardData {
    pub version: String,
    pub version_maps: Vec<String>,
    pub map: String,
}

#[derive(Serialize, Debug)]
pub struct AdventuresGetLeaderboardRequest {
    #[serde(rename = "type")]
    pub ty: AdventuresGetLeaderboardType,
    pub data: AdventuresGetLeaderboardData,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AdventuresRankItem {
    pub user: String,
    pub score: String,
    #[serde(rename = "uID")]
    pub uid: String,
    pub ai: String,
    #[serde(rename = "collabPartner")]
    pub collab_partner: String,
}

#[derive(Debug, Clone)]
pub struct AdventuresRankItemWithAvatar {
    pub user: String,
    pub avatar: String,
    pub score: u64,
    pub uid: String,
    pub badge: String,
}

impl AdventuresRankItemWithAvatar {
    fn make_badge_url_from(ai: &str, collab_partner: &str) -> String {
        BADGE_URL
            .replace("{ai}", &ai.to_lowercase())
            .replace("{collab}", &collab_partner.to_lowercase())
    }

    pub fn with_adventures_rank_item(rank_item: AdventuresRankItem, avatar_url: String) -> Result<Self, anyhow::Error> {
        Ok(Self {
            user: rank_item.user,
            avatar: avatar_url,
            score: u64::from_str_radix(&rank_item.score, 10)?,
            uid: rank_item.uid,
            badge: Self::make_badge_url_from(&rank_item.ai, &rank_item.collab_partner),
        })
    }
}
