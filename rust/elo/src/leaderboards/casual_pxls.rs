use tokio::sync::Mutex;

///! Casual Pxls leaderboard
use crate::_types::clptypes::{MessageTag, UserChatPerformance};
use crate::_types::leaderboardtypes::LeaderboardInnerState;
use crate::_types::CASUAL_ID_SUFFIX;
use crate::is_message_origin;
use crate::leaderboards::leaderboardtrait::AbstractLeaderboard;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct CasualPxls {
    state: HashMap<String, LeaderboardInnerState>,
}

impl AbstractLeaderboard for CasualPxls {
    fn new() -> Self {
        let mut out = Self {
            state: HashMap::new(),
        };
        out.read_initial_state();
        out
    }

    fn get_name(&self) -> String {
        "casual_pxls".to_string()
    }

    fn __get_state(&mut self) -> &mut HashMap<String, LeaderboardInnerState> {
        &mut self.state
    }

    fn calculate_score(&self, performance: &UserChatPerformance) -> Option<f32> {
        if is_message_origin!(performance, MessageTag::Pxls) {
            Some(*performance.metrics.get("score").unwrap_or(&0.0))
        } else {
            None
        }
    }

    fn save(&mut self) {
        let mut new_state = self
            .__get_state()
            .iter()
            .filter_map(|(k, state)| {
                if state.score != 0.0 {
                    let mut new_state = state.clone();
                    new_state.id = state.id.replace(CASUAL_ID_SUFFIX, "");
                    new_state.elo = state.score;
                    Some((k.to_string(), new_state))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();

        std::mem::swap(&mut self.state, &mut new_state);
        self.save_to_disk();
    }
}
