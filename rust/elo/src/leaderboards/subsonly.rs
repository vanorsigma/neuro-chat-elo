/*
Subs leaderboard
*/

use crate::_types::clptypes::{MessageTag, UserChatPerformance};
use crate::_types::leaderboardtypes::LeaderboardInnerState;
use crate::is_message_origin;
use crate::leaderboards::leaderboardtrait::AbstractLeaderboard;
use std::collections::{HashMap, HashSet};

#[derive(Default, Debug)]
pub struct SubsOnly {
    state: HashMap<String, LeaderboardInnerState>,
}

impl AbstractLeaderboard for SubsOnly {
    fn new(optout_list: &HashSet<String>) -> Self {
        let mut out = Self {
            state: HashMap::new(),
        };
        out.read_initial_state();
        out.cull_optout(optout_list);
        out
    }

    fn get_name(&self) -> String {
        "subs-only".to_string()
    }

    fn __get_state(&mut self) -> &mut HashMap<String, LeaderboardInnerState> {
        &mut self.state
    }

    fn calculate_score(&self, performance: &UserChatPerformance) -> Option<f32> {
        if !is_message_origin!(performance, MessageTag::Twitch) {
            return None;
        }

        if performance.metrics.contains_key("subs") {
            return Some(performance.metrics["subs"]);
        }
        Some(0.0)
    }
}
