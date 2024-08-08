/*
Copypasta leaders leaderboard
*/

use crate::_types::clptypes::UserChatPerformance;
use crate::_types::leaderboardtypes::LeaderboardInnerState;
use crate::leaderboards::leaderboardtrait::AbstractLeaderboard;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct CopypastaLeaders {
    state: HashMap<String, LeaderboardInnerState>,
}

impl AbstractLeaderboard for CopypastaLeaders {
    fn new() -> Self {
        let mut out = Self {
            state: HashMap::new(),
        };
        out.read_initial_state();
        out
    }

    fn get_name(&self) -> String {
        "copypasta".to_string()
    }

    fn __get_state(&mut self) -> &mut HashMap<String, LeaderboardInnerState> {
        &mut self.state
    }

    fn calculate_score(&self, performance: &UserChatPerformance) -> Option<f32> {
        if self.is_discord_message(performance) {
            return None
        } else {
            Some(*performance.metrics.get("copypasta").unwrap_or(&0.0))
        }
    }
}
