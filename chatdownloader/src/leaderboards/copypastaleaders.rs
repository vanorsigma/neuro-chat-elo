/*
Copypasta leaders leaderboard
*/

use crate::leaderboards::leaderboardtrait::AbstractLeaderboard;
use crate::_types::clptypes::UserChatPerformance;
use crate::_types::leaderboardtypes::LeaderboardInnerState;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct CopypastaLeaders {
    state: HashMap<String, LeaderboardInnerState>,
}

impl AbstractLeaderboard for CopypastaLeaders {
    fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }

    fn get_name(&self) -> &str {
        "copypasta"
    }

    fn __get_state(&mut self) -> &mut HashMap<String, LeaderboardInnerState> {
        &mut self.state
    }

    fn calculate_score(&self, performance: &UserChatPerformance) -> Option<f32> {
        Some(*performance.metrics.get("copypasta").unwrap_or(&0.0))
    }
}