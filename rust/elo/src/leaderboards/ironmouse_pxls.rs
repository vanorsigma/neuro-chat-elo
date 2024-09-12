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
            twitch_cache: Mutex::new(HashMap::new())
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
        } else if is_message_origin!(performance, MessageTag::Twitch) {
            tokio::task::block_in_place(|| {
                self.twitch_cache
                    .blocking_lock()
                    .insert("TWITCH-".to_string() + &performance.username, performance.clone())
            });
            None
        } else {
            None
        }
    }

    fn save(&mut self) {
        let mut new_values = tokio::task::block_in_place(|| {
            let twitch_cache_lock = self.twitch_cache.blocking_lock();
            self.state.clone().into_iter().filter_map(move |(key, mut state)| {
                if twitch_cache_lock.contains_key(&state.id) {
                    state.username = state.id.replace("TWITCH-", "");
                    state.elo = state.score;
                    state.avatar = twitch_cache_lock
                        .get(&state.id)
                        .unwrap()
                        .avatar
                        .clone();
                    state.id = state.id.replace("TWITCH-", "");
                    Some((key, state))
                } else {
                    None
                }
            })
                .collect::<HashMap<_, _>>()
        });

        // save to disk uses self.state
        std::mem::swap(&mut self.state, &mut new_values);
        self.save_to_disk();
    }
}
