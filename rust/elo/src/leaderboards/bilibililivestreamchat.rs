//! Bilibili chat leaderboard


use crate::_types::clptypes::UserChatPerformance;
use crate::_types::leaderboardtypes::LeaderboardInnerState;
use crate::leaderboards::leaderboardtrait::AbstractLeaderboard;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct BilibiliLivestreamChat {
    state: HashMap<String, LeaderboardInnerState>,
}

impl AbstractLeaderboard for BilibiliLivestreamChat {
    fn new() -> Self {
        let mut out = Self {
            state: HashMap::new(),
        };
        out.read_initial_state();
        out
    }

    fn get_name(&self) -> String {
        "bilibililivestreamchat".to_string()
    }

    fn __get_state(&mut self) -> &mut HashMap<String, LeaderboardInnerState> {
        &mut self.state
    }

    fn calculate_score(&self, performance: &UserChatPerformance) -> Option<f32> {
        if self.is_bilibili_message(performance) {
            Some(performance.metrics.values().sum())
        } else {
            None
        }
    }
}
