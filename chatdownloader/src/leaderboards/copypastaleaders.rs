/*
Copypasta leaders leaderboard
*/

use leaderboardtrait::AbstractLeaderboard;
use crate::_types::twitchtypes::UserChatPerformance;

pub struct ChatOnly {
    state: HashMap<String, LeaderboardInnerState>,
}

impl AbstractLeaderboard for ChatOnly {
    fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }

    fn get_name(&self) -> &str {
        "copypasta"
    }

    fn __get_state(&mut self) -> &mut HashMap<String, LeaderboardInnerState> {
        &mut self.state
    }

    fn calculate_score(&self, performance: &UserChatPerformance) -> Option<f32> {
        Some(performance.metrics.get("copypasta").unwrap_or(&0.0))
    }
}