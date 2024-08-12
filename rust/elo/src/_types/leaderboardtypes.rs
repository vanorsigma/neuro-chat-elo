/*
Publically accessible leaderboard types
*/
use super::clptypes::{BadgeInformation, UserChatPerformance};

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, prost::Message)]
pub struct LeaderboardExportItem {
    #[prost(string, tag = "1")]
    pub id: prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub rank: u32,
    #[prost(float, tag = "3")]
    pub elo: f32,
    #[prost(string, tag = "4")]
    pub username: prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub delta: i64,
    #[prost(string, tag = "6")]
    pub avatar: prost::alloc::string::String,
    #[prost(message, repeated, tag = "7")]
    pub badges: prost::alloc::vec::Vec<BadgeInformation>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaderboardExport {
    #[prost(message, repeated, tag = "1")]
    pub items: prost::alloc::vec::Vec<LeaderboardExportItem>,
}

#[derive(Debug, Clone)]
pub struct LeaderboardInnerState {
    pub id: String,
    pub username: String,
    pub avatar: String,
    pub badges: Option<Vec<BadgeInformation>>,
    pub previous_rank: Option<u32>,
    pub elo: f32,
    pub score: f32,
}

impl From<UserChatPerformance> for LeaderboardInnerState {
    fn from(performance: UserChatPerformance) -> Self {
        LeaderboardInnerState {
            id: performance.id,
            username: performance.username,
            avatar: performance.avatar,
            badges: None,
            previous_rank: None,
            elo: 1200.0,
            score: 0.0,
        }
    }
}

impl From<LeaderboardInnerState> for LeaderboardExportItem {
    fn from(value: LeaderboardInnerState) -> Self {
        LeaderboardExportItem {
            id: value.id,
            rank: 0,
            elo: value.elo,
            username: value.username,
            delta: 0,
            avatar: value.avatar,
            badges: value.badges.unwrap_or_default(),
        }
    }
}
