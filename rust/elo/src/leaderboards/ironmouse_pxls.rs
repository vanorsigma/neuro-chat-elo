use futures::FutureExt;
use tokio::sync::Mutex;
use twitch_utils::TwitchAPIWrapper;

///! (Special Event) Ironmouse Pxls Leaderboard
use crate::_types::clptypes::{MessageTag, UserChatPerformance};
use crate::_types::leaderboardtypes::LeaderboardInnerState;
use crate::_types::IRONMOUSE_ID_SUFFIX;
use crate::is_message_origin;
use crate::leaderboards::leaderboardtrait::AbstractLeaderboard;
use std::collections::HashMap;
use std::sync::Arc;

pub struct IronmousePxls {
    state: HashMap<String, LeaderboardInnerState>,
    twitch_api: Arc<TwitchAPIWrapper>,
}

impl IronmousePxls {
    pub fn new(twitch_api: Arc<TwitchAPIWrapper>) -> Self {
        let mut out = Self {
            state: HashMap::new(),
            twitch_api,
        };
        out.read_initial_state();
        out
    }
}

impl AbstractLeaderboard for IronmousePxls {
    fn get_name(&self) -> String {
        "ironmouse_pxls".to_string()
    }

    fn __get_state(&mut self) -> &mut HashMap<String, LeaderboardInnerState> {
        &mut self.state
    }

    fn calculate_score(&self, _performance: &UserChatPerformance) -> Option<f32> {
        None
    }

    async fn save(&mut self) {
        let mut new_state =
            futures::future::join_all(self.state.clone().into_iter().map(|(k, mut state)| async {
                self.twitch_api
                    .get_user_from_uid(state.id.clone())
                    .await
                    .map(|user| {
                        state.avatar = user.logo;
                        state.username = user.display_name;
                        (k, state)
                    })
                    .inspect_err(|e| {
                        log::error!(
                            "Cannot get avatar and username while updating ironmouse pxls. {}",
                            e
                        )
                    })
                    .ok()
            }))
            .await
            .into_iter()
            .filter_map(|o| o)
            .collect::<HashMap<_, _>>();

        std::mem::swap(&mut self.state, &mut new_state);
        self.save_to_disk();
    }
}
