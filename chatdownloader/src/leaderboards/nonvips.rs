/*
The non-VIPs leaderboard
*/

use crate::leaderboards::leaderboardtrait::AbstractLeaderboard;
use crate::_types::clptypes::UserChatPerformance;
use crate::_types::leaderboardtypes::LeaderboardInnerState;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct NonVIPS {
    state: HashMap<String, LeaderboardInnerState>,
}

impl AbstractLeaderboard for NonVIPS {
    fn new() -> Self {
        let mut out = Self {
            state: HashMap::new(),
        };
        out.read_initial_state();
        out
    }

    fn get_name(&self) -> &str {
        "nonvips"
    }

    fn __get_state(&mut self) -> &mut HashMap<String, LeaderboardInnerState> {
        &mut self.state
    }

    fn calculate_score(&self, performance: &UserChatPerformance) -> Option<f32> {
        if performance.metadata.contains_key("special_role") && *performance.metadata.get("special_role").unwrap().get_bool().unwrap() {
            return None;
        }
        return Some(performance.metrics.values().sum());
    }
}