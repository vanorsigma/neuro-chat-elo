use tokio::sync::Mutex;

///! (Special Event) Ironmouse Pxls Leaderboard
use crate::_types::clptypes::{MessageTag, UserChatPerformance};
use crate::_types::leaderboardtypes::LeaderboardInnerState;
use crate::is_message_origin;
use crate::leaderboards::leaderboardtrait::AbstractLeaderboard;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct IronmousePxls {
    state: HashMap<String, LeaderboardInnerState>,
    twitch_cache: Mutex<HashMap<String, UserChatPerformance>>,
}

impl AbstractLeaderboard for IronmousePxls {
    fn new() -> Self {
        let mut out = Self {
            state: HashMap::new(),
            twitch_cache: Mutex::new(HashMap::new()),
        };
        out.read_initial_state();
        out
    }

    fn get_name(&self) -> String {
        "ironmouse_pxls".to_string()
    }

    fn __get_state(&mut self) -> &mut HashMap<String, LeaderboardInnerState> {
        &mut self.state
    }

    fn calculate_score(&self, performance: &UserChatPerformance) -> Option<f32> {
        if is_message_origin!(performance, MessageTag::IronmousePixels) {
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
