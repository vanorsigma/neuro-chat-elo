/*
The non-VIPs leaderboard
*/

use crate::leaderboards::leaderboardtrait::AbstractLeaderboard;
use crate::_types::clptypes::UserChatPerformance;
use crate::_types::leaderboardtypes::LeaderboardInnerState;
use std::collections::HashMap;

pub struct ChatOnly {
    state: HashMap<String, LeaderboardInnerState>,
}

impl AbstractLeaderboard for ChatOnly {
    fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }

    fn get_name(&self) -> &str {
        "nonvips"
    }

    fn __get_state(&mut self) -> &mut HashMap<String, LeaderboardInnerState> {
        &mut self.state
    }

    fn calculate_score(&self, performance: &UserChatPerformance) -> Option<f32> {
        if performance.metadata.contains_key("special_role") && !performance.metadata["special_role"].is_null() {
            return None;
        }
        return Some(performance.metrics.values().sum());
    }
}