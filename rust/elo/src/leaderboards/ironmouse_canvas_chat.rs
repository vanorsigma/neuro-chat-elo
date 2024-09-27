use discord_utils::DiscordClient;

///! Ironmouse Canvas Chat
use crate::_types::clptypes::{MessageTag, UserChatPerformance};
use crate::_types::leaderboardtypes::LeaderboardInnerState;
use crate::is_message_origin;
use crate::leaderboards::leaderboardtrait::AbstractLeaderboard;
use std::collections::HashMap;
use std::sync::Arc;

pub struct IronmouseCanvasChat {
    state: HashMap<String, LeaderboardInnerState>,
    discord: Arc<DiscordClient>,
}

impl IronmouseCanvasChat {
    pub fn new(discord: Arc<DiscordClient>) -> Self {
        let mut out = Self {
            state: HashMap::new(),
            discord,
        };
        out.read_initial_state();
        out
    }
}

impl AbstractLeaderboard for IronmouseCanvasChat {
    fn get_name(&self) -> String {
        "ironmousecanvaschat".to_string()
    }

    fn __get_state(&mut self) -> &mut HashMap<String, LeaderboardInnerState> {
        &mut self.state
    }

    fn calculate_score(&self, _performance: &UserChatPerformance) -> Option<f32> {
        None
    }

    async fn save(&mut self) {
        let mut new_state = futures::future::join_all(
            self.state
                .clone()
                .into_iter()
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
