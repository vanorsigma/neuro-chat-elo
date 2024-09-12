use tokio::sync::Mutex;

///! Casual Pxls leaderboard
use crate::_types::clptypes::{MessageTag, UserChatPerformance};
use crate::_types::leaderboardtypes::LeaderboardInnerState;
use crate::is_message_origin;
use crate::leaderboards::leaderboardtrait::AbstractLeaderboard;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct CasualPxls {
    state: HashMap<String, LeaderboardInnerState>,
    discord_cache: Mutex<HashMap<String, UserChatPerformance>>,
}

impl AbstractLeaderboard for CasualPxls {
    fn new() -> Self {
        let mut out = Self {
            state: HashMap::new(),
            discord_cache: Mutex::new(HashMap::new()),
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
        } else if is_message_origin!(performance, MessageTag::Discord) {
            tokio::task::block_in_place(|| {
                self.discord_cache
                    .blocking_lock()
                    .insert("DISCORD-".to_string() + &performance.username, performance.clone())
            });
            None
        } else {
            None
        }
    }

    fn save(&mut self) {
        let mut new_values = tokio::task::block_in_place(|| {
            let discord_cache_lock = self.discord_cache.blocking_lock();
            self.state.clone().into_iter().filter_map(move |(key, mut state)| {
                if discord_cache_lock.contains_key(&state.id) {
                    state.username = state.id.replace("DISCORD-", "");
                    state.elo = state.score;
                    state.avatar = discord_cache_lock
                        .get(&state.id)
                        .unwrap()
                        .avatar
                        .clone();
                    state.id = state.id.replace("DISCORD-", "");
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
