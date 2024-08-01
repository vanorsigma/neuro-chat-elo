/*
The non-VIPs leaderboard
*/

use crate::_types::clptypes::UserChatPerformance;
use crate::_types::leaderboardtypes::LeaderboardInnerState;
use crate::leaderboards::leaderboardtrait::AbstractLeaderboard;
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

    fn get_name(&self) -> String {
        "nonvips".to_string()
    }

    fn __get_state(&mut self) -> &mut HashMap<String, LeaderboardInnerState> {
        &mut self.state
    }

    fn calculate_score(&self, performance: &UserChatPerformance) -> Option<f32> {
        if !self.is_discord_message(performance) {
            return None;
        }

        if let Some(special_role) = performance.metadata.get("special_role") {
            if *special_role.get_bool().unwrap_or(&false) {
                return None;
            }
        }
        Some(performance.metrics.values().sum())
    }
}
