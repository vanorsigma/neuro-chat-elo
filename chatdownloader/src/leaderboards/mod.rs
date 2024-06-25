mod leaderboardtrait;
mod bitsonly;
mod chatonly;
mod copypastaleaders;
mod overall;
mod subsonly;
mod nonvips;

use crate::leaderboards::leaderboardtrait::AbstractLeaderboard;

#[allow(dead_code)]
pub fn get_leaderboards() -> Vec<Box<dyn AbstractLeaderboard>> {
    vec![
        Box::new(bitsonly::BitsOnly::new()),
        Box::new(chatonly::ChatOnly::new()),
        Box::new(copypastaleaders::CopypastaLeaders::new()),
        Box::new(overall::Overall::new()),
        Box::new(subsonly::SubsOnly::new()),
        Box::new(nonvips::NonVIPS::new()),
    ]
}