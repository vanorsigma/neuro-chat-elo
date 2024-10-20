/*
Publically accessible leaderboard types
*/

// Includes the protobuf types defined in models/leaderboardExportTypes.proto
include!(concat!(env!("OUT_DIR"), "/leaderboard_export_types.rs"));

impl From<Vec<LeaderboardExportItem>> for LeaderboardExport {
    fn from(items: Vec<LeaderboardExportItem>) -> Self {
        LeaderboardExport { items }
    }
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

pub fn export_item_to_inner_state(item: LeaderboardExportItem) -> LeaderboardInnerState {
    LeaderboardInnerState {
        id: item.id,
        username: item.username,
        avatar: item.avatar,
        badges: Some(item.badges),
        previous_rank: Some(item.rank),
        elo: item.elo,
        score: 1200.0,
    }
}
