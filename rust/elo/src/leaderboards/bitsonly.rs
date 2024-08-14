/*
Bits leaderboard
*/

use crate::_types::clptypes::UserChatPerformance;
use crate::_types::leaderboardtypes::LeaderboardInnerState;
use crate::leaderboards::leaderboardtrait::AbstractLeaderboard;
use std::collections::HashMap;

use super::Leaderboard;

const K: f32 = 2.0;

#[derive(Default, Debug)]
pub struct BitsOnly {
    state: HashMap<String, LeaderboardInnerState>,
}

impl AbstractLeaderboard for BitsOnly {
    fn new() -> Self {
        let mut out = Self {
            state: HashMap::new(),
        };
        out.read_initial_state();
        out
    }

    fn get_name(&self) -> String {
        "bits-only".to_string()
    }

    fn __get_state(&mut self) -> &mut HashMap<String, LeaderboardInnerState> {
        &mut self.state
    }

    fn calculate_score(&self, performance: &UserChatPerformance) -> Option<f32> {
        Some(performance.metrics.get("bits").unwrap_or(&0.0) * K)
    }
}

impl Into<Leaderboard> for BitsOnly {
    fn into(self) -> Leaderboard {
        Leaderboard::BitsOnly(self)
    }
}
