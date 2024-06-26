/*
Bits leaderboard
*/

use crate::leaderboards::leaderboardtrait::AbstractLeaderboard;
use crate::_types::clptypes::UserChatPerformance;
use crate::_types::leaderboardtypes::LeaderboardInnerState;
use std::collections::HashMap;

const K: f32 = 2.0;

#[derive(Default, Debug)]
pub struct BitsOnly {
    state: HashMap<String, LeaderboardInnerState>,
}

impl AbstractLeaderboard for BitsOnly {
    fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }

    fn get_name(&self) -> &str {
        "bits-only"
    }

    fn __get_state(&mut self) -> &mut HashMap<String, LeaderboardInnerState> {
        &mut self.state
    }

    fn calculate_score(&self, performance: &UserChatPerformance) -> Option<f32> {
        Some(performance.metrics.get("bits").unwrap_or(&0.0) * K)
    }
}