/*
Top Emotes leaderboard
*/

use crate::_types::clptypes::{MessageTag, UserChatPerformance};
use crate::_types::leaderboardtypes::LeaderboardInnerState;
use crate::is_message_origin;
use crate::leaderboards::leaderboardtrait::AbstractLeaderboard;
use std::collections::HashMap;

const K: f32 = 1.0;

#[derive(Default, Debug)]
pub struct TopEmote {
    state: HashMap<String, LeaderboardInnerState>,
}

impl TopEmote {
    pub fn new() -> Self {
        let mut out = Self {
            state: HashMap::new(),
        };
        out.read_initial_state();
        out
    }
}

impl AbstractLeaderboard for TopEmote {
    fn get_name(&self) -> String {
        "top-emote".to_string()
    }

    fn __get_state(&mut self) -> &mut HashMap<String, LeaderboardInnerState> {
        &mut self.state
    }

    fn calculate_score(&self, performance: &UserChatPerformance) -> Option<f32> {
        if is_message_origin!(performance, MessageTag::Emote) {
            Some(*performance.metrics.get("emote_use").unwrap_or(&0.0) * K)
        } else {
            None
        }
    }
}
