///! Partners leaderboard
use crate::_types::clptypes::{MetadataTypes, UserChatPerformance};
use crate::_types::leaderboardtypes::LeaderboardInnerState;
use crate::leaderboards::leaderboardtrait::AbstractLeaderboard;
use std::collections::HashMap;

const PARTNER_DESC: &str = "partner";
const K: f32 = 2.0;

#[derive(Default, Debug)]
pub struct PartnersOnly {
    state: HashMap<String, LeaderboardInnerState>,
}

impl AbstractLeaderboard for PartnersOnly {
    fn new() -> Self {
        let mut out = Self {
            state: HashMap::new(),
        };
        out.read_initial_state();
        out
    }

    fn get_name(&self) -> String {
        "partners-only".to_string()
    }

    fn __get_state(&mut self) -> &mut HashMap<String, LeaderboardInnerState> {
        &mut self.state
    }

    fn calculate_score(&self, performance: &UserChatPerformance) -> Option<f32> {
        if let Some(MetadataTypes::BadgeList(badge_list)) = performance.metadata.get("badges") {
            badge_list
                .iter()
                .find(|badge| badge.description == PARTNER_DESC)
                .map(|_| K)
        } else {
            None
        }
    }
}
