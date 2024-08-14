mod bitsonly;
mod chatonly;
mod copypastaleaders;
mod leaderboardtrait;
mod nonvips;
mod overall;
mod subsonly;

use log::error;
use tokio::sync::broadcast;

use crate::{
    _types::clptypes::UserChatPerformance, leaderboards::leaderboardtrait::AbstractLeaderboard,
};

async fn calc_leaderboard(
    mut leaderboard: Leaderboard,
    mut reciever: broadcast::Receiver<UserChatPerformance>,
) {
    loop {
        let user_chat_performance: UserChatPerformance = match reciever.recv().await {
            Ok(user_chat_performance) => user_chat_performance,
            Err(_) => break,
        };

        leaderboard
            .get_as_abstract_mut()
            .update_leaderboard(user_chat_performance);

        let mut leaders = leaderboard
            .get_as_abstract_mut()
            .__get_state()
            .iter()
            .collect::<Vec<_>>();

        leaders.sort_by(|(_, l), (_, r)| {
            l.elo
                .partial_cmp(&r.elo)
                .unwrap_or(std::cmp::Ordering::Less)
                .reverse()
        });
    }

    leaderboard.get_as_abstract_mut().save();
}

pub enum Leaderboard {
    BitsOnly(bitsonly::BitsOnly),
    ChatOnly(chatonly::ChatOnly),
    Copypasta(copypastaleaders::CopypastaLeaders),
    NonVips(nonvips::NonVIPS),
    Overall(overall::Overall),
    SubsOnly(subsonly::SubsOnly),
}

impl Leaderboard {
    fn get_as_abstract(&self) -> &dyn AbstractLeaderboard {
        match self {
            Leaderboard::BitsOnly(v) => v,
            Leaderboard::ChatOnly(v) => v,
            Leaderboard::Copypasta(v) => v,
            Leaderboard::NonVips(v) => v,
            Leaderboard::Overall(v) => v,
            Leaderboard::SubsOnly(v) => v,
        }
    }

    fn get_as_abstract_mut(&mut self) -> &mut dyn AbstractLeaderboard {
        match self {
            Leaderboard::BitsOnly(v) => v,
            Leaderboard::ChatOnly(v) => v,
            Leaderboard::Copypasta(v) => v,
            Leaderboard::NonVips(v) => v,
            Leaderboard::Overall(v) => v,
            Leaderboard::SubsOnly(v) => v,
        }
    }
}

pub struct LeaderboardProcessorBuilder {
    leaderboards: Vec<Leaderboard>,
    broadcast_capacity: usize,
}

impl Default for LeaderboardProcessorBuilder {
    fn default() -> Self {
        Self {
            leaderboards: Vec::new(),
            broadcast_capacity: 100_000,
        }
    }
}

impl LeaderboardProcessorBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn all_leaderboards() -> Self {
        let mut builder = Self::new();
        builder.add_leaderboard(bitsonly::BitsOnly::new());
        builder.add_leaderboard(chatonly::ChatOnly::new());
        builder.add_leaderboard(copypastaleaders::CopypastaLeaders::new());
        builder.add_leaderboard(nonvips::NonVIPS::new());
        builder.add_leaderboard(overall::Overall::new());
        builder.add_leaderboard(subsonly::SubsOnly::new());
        builder
    }

    pub fn add_leaderboard<L: Into<Leaderboard>>(&mut self, leaderboard: L) {
        self.leaderboards.push(leaderboard.into());
    }

    pub fn set_broadcast_capacity(&mut self, capacity: usize) {
        self.broadcast_capacity = capacity;
    }

    pub fn spawn(self) -> LeaderboardProcessor {
        let mut joinset = tokio::task::JoinSet::new();
        let (send, recv) = broadcast::channel(self.broadcast_capacity);

        for leaderboard in self.leaderboards {
            let reciever = recv.resubscribe();

            joinset.spawn(async move {
                calc_leaderboard(leaderboard, reciever).await;
            });
        }

        LeaderboardProcessor {
            sender: send,
            join_set: joinset,
        }
    }
}

pub struct LeaderboardProcessor {
    sender: broadcast::Sender<UserChatPerformance>,
    join_set: tokio::task::JoinSet<()>,
}

impl LeaderboardProcessor {
    pub fn send_performance(&self, performance: UserChatPerformance) {
        self.sender.send(performance).unwrap();
    }

    pub async fn finish(mut self) {
        // this needs to be dropped so each leaderboard processing task will finish consuming messages
        // then save the leaderboard
        drop(self.sender);

        while !self.join_set.is_empty() {
            self.join_set.join_next().await;
        }
    }
}

pub async fn send_performances(
    sender: broadcast::Sender<UserChatPerformance>,
    performances: Vec<UserChatPerformance>,
) {
    for performance in performances {
        if let Err(e) = sender.send(performance) {
            error!("Error sending performance to leaderboards: {}", e);
        }
    }
}
