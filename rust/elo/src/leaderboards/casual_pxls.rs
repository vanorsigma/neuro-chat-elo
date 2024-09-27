use discord_utils::DiscordClient;
use tokio::sync::Mutex;

///! Casual Pxls leaderboard
use crate::_types::clptypes::{MessageTag, UserChatPerformance};
use crate::_types::leaderboardtypes::LeaderboardInnerState;
use crate::_types::CASUAL_ID_SUFFIX;
use crate::is_message_origin;
use crate::leaderboards::leaderboardtrait::AbstractLeaderboard;
use std::collections::HashMap;
use std::sync::Arc;

use super::leaderboardtrait::STARTING_ELO;

pub struct CasualPxls {
    state: HashMap<String, LeaderboardInnerState>,
    discord: Arc<DiscordClient>,
}

impl CasualPxls {
    pub fn add_suffix_to_all_state(mut self) -> Self {
        self.state = self
            .state
            .into_iter()
            .map(|(k, mut val)| {
                val.id = val.id.clone() + CASUAL_ID_SUFFIX;
                (k + CASUAL_ID_SUFFIX, val)
            })
            .collect();
        self
    }

    pub fn new(discord: Arc<DiscordClient>) -> Self {
        let mut out = Self {
            state: HashMap::new(),
            discord,
        };
        out.read_initial_state();
        out.add_suffix_to_all_state()
    }
}

impl AbstractLeaderboard for CasualPxls {
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

    async fn save(&mut self) {
        let mut new_state = futures::future::join_all(
            self.state
                .clone()
                .into_iter()
                .map(|(k, state)| {
                    let mut new_state = state.clone();
                    new_state.id = state.id.replace(CASUAL_ID_SUFFIX, "");

                    // hack: if we see the starting elo, it means that we've
                    // never seen this user in our lives
                    if state.elo == STARTING_ELO {
                        new_state.elo = state.score
                    } else {
                        new_state.elo = state.score.max(state.elo);
                    }

                    (k.to_string(), new_state)
                })
                .map(|(k, mut state)| async {
                    self.discord
                        .cached_get_profile_for_user_id(state.id.clone())
                        .await
                        .map(|user| {
                            state.avatar = user.avatar;
                            state.username = user.global_name;
                            (k, state)
                        })
                        .inspect_err(|e| log::error!("cannot get discord username {}", e))
                        .ok()
                }),
        )
        .await
        .into_iter()
        .filter_map(|o| o)
        .collect::<HashMap<_, _>>();

        std::mem::swap(&mut self.state, &mut new_state);
        self.save_to_disk();
    }
}
