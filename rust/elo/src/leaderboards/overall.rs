/*
The overall leaderboard
*/

use crate::_types::clptypes::UserChatPerformance;
use crate::_types::leaderboardtypes::LeaderboardInnerState;
use crate::leaderboards::leaderboardtrait::AbstractLeaderboard;
use std::collections::HashMap;

use super::Leaderboard;

#[derive(Default, Debug)]
pub struct Overall {
    state: HashMap<String, LeaderboardInnerState>,
}

impl AbstractLeaderboard for Overall {
    fn new() -> Self {
        let mut out = Self {
            state: HashMap::new(),
        };
        out.read_initial_state();
        out
    }

    fn get_name(&self) -> String {
        "overall".to_string()
    }

    fn __get_state(&mut self) -> &mut HashMap<String, LeaderboardInnerState> {
        &mut self.state
    }

    fn calculate_score(&self, performance: &UserChatPerformance) -> Option<f32> {
        return Some(performance.metrics.values().sum());
    }
}

impl Into<Leaderboard> for Overall {
    fn into(self) -> Leaderboard {
        Leaderboard::Overall(self)
    }
}
