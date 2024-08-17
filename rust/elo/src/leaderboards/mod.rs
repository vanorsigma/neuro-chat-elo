mod bitsonly;
mod chatonly;
mod copypastaleaders;
mod leaderboardtrait;
mod nonvips;
mod overall;
mod subsonly;

use std::collections::HashMap;

use log::error;
use tokio::sync::{broadcast, mpsc};

use crate::{
    _types::{clptypes::UserChatPerformance, leaderboardtypes::LeaderboardExportItem},
    leaderboards::leaderboardtrait::AbstractLeaderboard,
};

async fn calc_leaderboard(
    mut leaderboard: Leaderboard,
    mut reciever: broadcast::Receiver<UserChatPerformance>,
    mut incoming_peek_recv: broadcast::Receiver<()>,
    outgoing_peek_send: mpsc::Sender<Vec<LeaderboardExportItem>>,
) {
    loop {
        let action = tokio::select! {
            msg = reciever.recv() => Action::Performance(msg),
            _ = incoming_peek_recv.recv() => Action::Peek,
        };

        match action {
            Action::Performance(performance) => {
                let user_chat_performance = match performance {
                    Ok(user_chat_performance) => user_chat_performance,
                    Err(_) => break,
                };

                leaderboard
                    .get_as_abstract_mut()
                    .update_leaderboard(user_chat_performance);
            }
            Action::Peek => {
                let leaders = leaderboard.get_as_abstract_mut().generate_export();

                outgoing_peek_send.send(leaders).await.unwrap();
            }
        }
    }

    leaderboard.get_as_abstract_mut().save();
}

enum Action {
    Performance(Result<UserChatPerformance, broadcast::error::RecvError>),
    Peek,
}

macro_rules! leaderboard_gen {
    ($( $enum_entry_name:ident => $leaderboard_type:ty ),* $(,)?) => {
        pub enum Leaderboard {
            $(
                $enum_entry_name($leaderboard_type),
            )*
        }

        impl Leaderboard {
            pub fn get_as_abstract(&self) -> &dyn AbstractLeaderboard {
                match self {
                    $(
                        Leaderboard::$enum_entry_name(v) => v,
                    )*
                }
            }

            pub fn get_as_abstract_mut(&mut self) -> &mut dyn AbstractLeaderboard {
                match self {
                    $(
                        Leaderboard::$enum_entry_name(v) => v,
                    )*
                }
            }
        }

        impl LeaderboardProcessorBuilder {
            pub fn all_leaderboards() -> Self {
                let mut builder = Self::new();
                $(
                    builder.add_leaderboard(<$leaderboard_type>::new());
                )*
                builder
            }
        }

        $(
            impl From<$leaderboard_type> for Leaderboard {
                fn from(value: $leaderboard_type) -> Leaderboard {
                    Leaderboard::$enum_entry_name(value)
                }
            }
        )*
    };
}

leaderboard_gen!(
    BitsOnly => bitsonly::BitsOnly,
    ChatOnly => chatonly::ChatOnly,
    Copypasta => copypastaleaders::CopypastaLeaders,
    NonVips => nonvips::NonVIPS,
    Overall => overall::Overall,
    SubsOnly => subsonly::SubsOnly,
);

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

    pub fn add_leaderboard<L: Into<Leaderboard>>(&mut self, leaderboard: L) {
        self.leaderboards.push(leaderboard.into());
    }

    pub fn set_broadcast_capacity(&mut self, capacity: usize) {
        self.broadcast_capacity = capacity;
    }

    pub fn spawn(self) -> LeaderboardProcessor {
        let mut joinset = tokio::task::JoinSet::new();
        let (send, recv) = broadcast::channel(self.broadcast_capacity);
        let (incoming_peek_send, incoming_peek_recv) = broadcast::channel(10);

        let mut leaderboard_peek_recvs = HashMap::new();

        for leaderboard in self.leaderboards {
            let (outgoing_peek_send, outgoing_peek_recv) = mpsc::channel(10);
            let incoming_peek_recv = incoming_peek_recv.resubscribe();
            let reciever = recv.resubscribe();

            leaderboard_peek_recvs
                .insert(leaderboard.get_as_abstract().get_name(), outgoing_peek_recv);

            joinset.spawn(async move {
                calc_leaderboard(
                    leaderboard,
                    reciever,
                    incoming_peek_recv,
                    outgoing_peek_send,
                )
                .await;
            });
        }

        LeaderboardProcessor {
            sender: send,
            join_set: joinset,
            leaderboard_peek_send: incoming_peek_send,
            leaderboard_peek_recvs,
        }
    }
}

pub struct LeaderboardProcessor {
    sender: broadcast::Sender<UserChatPerformance>,
    leaderboard_peek_send: broadcast::Sender<()>,
    leaderboard_peek_recvs: HashMap<String, mpsc::Receiver<Vec<LeaderboardExportItem>>>,
    join_set: tokio::task::JoinSet<()>,
}

impl LeaderboardProcessor {
    pub fn send_performance(&self, performance: UserChatPerformance) {
        self.sender.send(performance).unwrap();
    }

    pub async fn peek(&mut self) -> HashMap<String, Vec<LeaderboardExportItem>> {
        self.leaderboard_peek_send.send(()).unwrap();

        let mut out = HashMap::new();

        for (name, leaderboard) in &mut self.leaderboard_peek_recvs {
            out.insert(name.to_string(), leaderboard.recv().await.unwrap());
        }

        out
    }

    pub async fn finish(mut self) {
        // this needs to be dropped so each leaderboard processing task will finish consuming messages
        // then save the leaderboard
        drop(self.sender);
        drop(self.leaderboard_peek_send);

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
