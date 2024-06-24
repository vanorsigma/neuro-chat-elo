/*
Bits leaderboard
*/

use leaderboardtrait::AbstractLeaderboard;
use crate::_types::twitchtypes::UserChatPerformance;

const K: f32 = 2.0;

pub struct BitsOnly {
    state: HashMap<String, LeaderboardInnerState>,
}

impl AbstractLeaderboard for BitsOnly {
    fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }

    fn get_name(&self) -> &str {
        "bits"
    }

    fn __get_state(&mut self) -> &mut HashMap<String, LeaderboardInnerState> {
        &mut self.state
    }

    fn calculate_score(&self, performance: &UserChatPerformance) -> Option<f32> {
        Some(performance.metrics.get("bits").unwrap_or(&0.0) * K)
    }
}