///! Adventures leaderboard for the Farm map
use crate::_types::clptypes::{MessageTag, UserChatPerformance};
use crate::_types::leaderboardtypes::LeaderboardInnerState;
use crate::is_message_origin;
use crate::leaderboards::leaderboardtrait::AbstractLeaderboard;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct AdventuresFarm {
    state: HashMap<String, LeaderboardInnerState>,
}

impl AdventuresFarm {
    pub fn new() -> Self {
        let mut out = Self {
            state: HashMap::new(),
        };
        out.read_initial_state();
        out
    }
}

impl AbstractLeaderboard for AdventuresFarm {
    fn get_name(&self) -> String {
        "adventures_farm".to_string()
    }

    fn __get_state(&mut self) -> &mut HashMap<String, LeaderboardInnerState> {
        &mut self.state
    }

    fn calculate_score(&self, performance: &UserChatPerformance) -> Option<f32> {
        if is_message_origin!(performance, MessageTag::Adventures) {
            Some(*performance.metrics.get("score").unwrap_or(&0.0))
        } else {
            None
        }
    }

    async fn save(&mut self) {
        self.__get_state()
            .values_mut()
            .for_each(|state| state.elo = state.score);
        self.save_to_disk();
    }
}
