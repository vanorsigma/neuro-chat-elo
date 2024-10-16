mod bitsonly;
mod chatonly;
mod copypastaleaders;
mod discordlivestreamchat;
mod leaderboardtrait;
mod nonvips;
mod overall;
mod partnersonly;
mod subsonly;
mod topemote;

use futures::join;

use log::error;
use tokio::sync::broadcast;

use crate::{
    _types::clptypes::UserChatPerformance, leaderboards::leaderboardtrait::AbstractLeaderboard,
};

async fn calc_leaderboard<M: AbstractLeaderboard + Sync + Send + 'static>(
    leaderboard: &mut M,
    mut reciever: broadcast::Receiver<UserChatPerformance>,
) {
    /*
    Update the leaderboard based on chat messages sent by a tokio broadcast channel
    */
    loop {
        let user_chat_performance: UserChatPerformance = match reciever.recv().await {
            Ok(user_chat_performance) => user_chat_performance,
            Err(_) => break,
        };
        leaderboard.update_leaderboard(user_chat_performance);
    }
    leaderboard.save();
}

pub struct LeaderboardProcessor {
    bitsonly: bitsonly::BitsOnly,
    chatonly: chatonly::ChatOnly,
    copypasta: copypastaleaders::CopypastaLeaders,
    nonvips: nonvips::NonVIPS,
    overall: overall::Overall,
    subsonly: subsonly::SubsOnly,
    topemote: topemote::TopEmote,
    discordlivestreamchat: discordlivestreamchat::DiscordLivestreamChat,
    partnersonly: partnersonly::PartnersOnly,
}

impl LeaderboardProcessor {
    pub fn new() -> Self {
        let bitsonly = bitsonly::BitsOnly::new();
        let chatonly = chatonly::ChatOnly::new();
        let copypasta = copypastaleaders::CopypastaLeaders::new();
        let nonvips = nonvips::NonVIPS::new();
        let overall = overall::Overall::new();
        let subsonly = subsonly::SubsOnly::new();
        let topemote = topemote::TopEmote::new();
        let discordlivestreamchat = discordlivestreamchat::DiscordLivestreamChat::new();
        let partnersonly = partnersonly::PartnersOnly::new();

        Self {
            bitsonly,
            chatonly,
            copypasta,
            nonvips,
            overall,
            subsonly,
            topemote,
            discordlivestreamchat,
            partnersonly,
        }
    }

    pub async fn run(&mut self, performances: Vec<UserChatPerformance>) {
        let (broadcast_sender, broadcast_reciever) = broadcast::channel(100000);

        join!(
            send_performances(broadcast_sender, performances),
            calc_leaderboard(&mut self.bitsonly, broadcast_reciever.resubscribe()),
            calc_leaderboard(&mut self.chatonly, broadcast_reciever.resubscribe()),
            calc_leaderboard(&mut self.copypasta, broadcast_reciever.resubscribe()),
            calc_leaderboard(&mut self.nonvips, broadcast_reciever.resubscribe()),
            calc_leaderboard(&mut self.overall, broadcast_reciever.resubscribe()),
            calc_leaderboard(&mut self.subsonly, broadcast_reciever.resubscribe()),
            calc_leaderboard(&mut self.topemote, broadcast_reciever.resubscribe()),
            calc_leaderboard(
                &mut self.discordlivestreamchat,
                broadcast_reciever.resubscribe()
            ),
            calc_leaderboard(&mut self.partnersonly, broadcast_reciever.resubscribe())
        );
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
