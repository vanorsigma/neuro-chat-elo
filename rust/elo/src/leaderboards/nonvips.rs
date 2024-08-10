/*
The non-VIPs leaderboard
*/

use crate::_types::clptypes::{MessageTag, UserChatPerformance};
use crate::_types::leaderboardtypes::LeaderboardInnerState;
use crate::is_message_origin;
use crate::leaderboards::leaderboardtrait::AbstractLeaderboard;
use std::collections::{HashMap, HashSet};

#[derive(Default, Debug)]
pub struct NonVIPS {
    state: HashMap<String, LeaderboardInnerState>,
}

impl AbstractLeaderboard for NonVIPS {
    fn new(optout_list: &HashSet<String>) -> Self {
        let mut out = Self {
            state: HashMap::new(),
        };
        out.read_initial_state();
        out.cull_optout(optout_list);
        out
    }

    fn get_name(&self) -> String {
        "nonvips".to_string()
    }

    fn __get_state(&mut self) -> &mut HashMap<String, LeaderboardInnerState> {
        &mut self.state
    }

    fn calculate_score(&self, performance: &UserChatPerformance) -> Option<f32> {
        if is_message_origin!(performance, MessageTag::Discord) {
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
