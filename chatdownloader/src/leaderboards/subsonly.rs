/*
Subs leaderboard
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
        "subs-only"
    }

    fn __get_state(&mut self) -> &mut HashMap<String, LeaderboardInnerState> {
        &mut self.state
    }

    fn calculate_score(&self, performance: &UserChatPerformance) -> Option<f32> {
        if performance.metrics.contains_key("subs") && !performance.metadata["subs"].is_null() {
            return Some(performance.metrics["subs"]);
        }
        return None;
    }
}