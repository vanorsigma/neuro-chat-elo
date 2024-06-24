/*
Publically accessible leaderboard types
*/

use serde::{Deserialize, Serialize};
use super::clptypes::BadgeInformation;

#[derive(Deserialize, Serialize)]
pub struct LeaderboardExportItem {
    pub id: String,
    pub rank: u16,
    pub elo: f32,
    pub username: String,
    pub delta: f32,
    pub avatar: String,
    pub badges: Option<Vec<BadgeInformation>>
}

#[derive(Deserialize, Serialize)]
pub struct LeaderboardInnerState {
    pub id: String,
    pub username: String,
    pub avatar: String,
    pub badges: Option<Vec<BadgeInformation>>,
    pub previous_rank: Option<u16>,
    pub elo: f32,
    pub score: f32
}