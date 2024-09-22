mod adventures_farm;
mod bilibililivestreamchat;
mod bitsonly;
mod casual_pxls;
mod chatonly;
mod copypastaleaders;
mod discordlivestreamchat;
mod ironmouse_pxls;
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

async fn calc_leaderboard<L: AbstractLeaderboard + Sync + Send + 'static>(
    leaderboard: &mut WithReceiver<L>,
) {
    /*
    Update the leaderboard based on chat messages sent by a tokio broadcast channel
    */
    loop {
        let user_chat_performance: UserChatPerformance = match leaderboard.receiver.recv().await {
            Ok(user_chat_performance) => user_chat_performance,
            Err(_) => break,
        };
        leaderboard
            .leaderboard
            .update_leaderboard(user_chat_performance);
    }
    leaderboard.leaderboard.save();
}

struct WithReceiver<L: AbstractLeaderboard> {
    pub leaderboard: L,
    pub receiver: broadcast::Receiver<UserChatPerformance>,
}

impl<L: AbstractLeaderboard> WithReceiver<L> {
    fn new(leaderboard: L, receiver: broadcast::Receiver<UserChatPerformance>) -> Self
    where
        Self: Sized,
    {
        Self {
            leaderboard,
            receiver,
        }
    }
}

pub struct LeaderboardProcessor {
    tx: Option<broadcast::Sender<UserChatPerformance>>,
    bitsonly: WithReceiver<bitsonly::BitsOnly>,
    chatonly: WithReceiver<chatonly::ChatOnly>,
    copypasta: WithReceiver<copypastaleaders::CopypastaLeaders>,
    nonvips: WithReceiver<nonvips::NonVIPS>,
    overall: WithReceiver<overall::Overall>,
    subsonly: WithReceiver<subsonly::SubsOnly>,
    topemote: WithReceiver<topemote::TopEmote>,
    discordlivestreamchat: WithReceiver<discordlivestreamchat::DiscordLivestreamChat>,
    partnersonly: WithReceiver<partnersonly::PartnersOnly>,
    bilibililivestreamchat: WithReceiver<bilibililivestreamchat::BilibiliLivestreamChat>,
    adventures_farm: WithReceiver<adventures_farm::AdventuresFarm>,
    casual_pxls: WithReceiver<casual_pxls::CasualPxls>,
    ironmouse_pxls: WithReceiver<ironmouse_pxls::IronmousePxls>,
}

impl LeaderboardProcessor {
    pub fn new() -> Self {
        let (tx, rx) = broadcast::channel(100000);

        let bitsonly = WithReceiver::new(bitsonly::BitsOnly::new(), rx.resubscribe());
        let chatonly = WithReceiver::new(chatonly::ChatOnly::new(), rx.resubscribe());
        let copypasta =
            WithReceiver::new(copypastaleaders::CopypastaLeaders::new(), rx.resubscribe());
        let nonvips = WithReceiver::new(nonvips::NonVIPS::new(), rx.resubscribe());
        let overall = WithReceiver::new(overall::Overall::new(), rx.resubscribe());
        let subsonly = WithReceiver::new(subsonly::SubsOnly::new(), rx.resubscribe());
        let topemote = WithReceiver::new(topemote::TopEmote::new(), rx.resubscribe());
        let discordlivestreamchat = WithReceiver::new(
            discordlivestreamchat::DiscordLivestreamChat::new(),
            rx.resubscribe(),
        );
        let partnersonly = WithReceiver::new(partnersonly::PartnersOnly::new(), rx.resubscribe());
        let bilibililivestreamchat = WithReceiver::new(
            bilibililivestreamchat::BilibiliLivestreamChat::new(),
            rx.resubscribe(),
        );
        let adventures_farm =
            WithReceiver::new(adventures_farm::AdventuresFarm::new(), rx.resubscribe());
        let casual_pxls = WithReceiver::new(casual_pxls::CasualPxls::new(), rx.resubscribe());
        let ironmouse_pxls =
            WithReceiver::new(ironmouse_pxls::IronmousePxls::new(), rx.resubscribe());

        Self {
            tx: Some(tx),
            bitsonly,
            chatonly,
            copypasta,
            nonvips,
            overall,
            subsonly,
            topemote,
            discordlivestreamchat,
            partnersonly,
            bilibililivestreamchat,
            adventures_farm,
            casual_pxls,
            ironmouse_pxls,
        }
    }

    pub async fn run(&mut self, performances: Vec<UserChatPerformance>) {
        join!(
            send_performances(&mut self.tx, performances),
            calc_leaderboard(&mut self.bitsonly),
            calc_leaderboard(&mut self.chatonly),
            calc_leaderboard(&mut self.copypasta),
            calc_leaderboard(&mut self.nonvips),
            calc_leaderboard(&mut self.overall),
            calc_leaderboard(&mut self.subsonly),
            calc_leaderboard(&mut self.topemote),
            calc_leaderboard(
                &mut self.discordlivestreamchat
            ),
            calc_leaderboard(&mut self.partnersonly),
            calc_leaderboard(
                &mut self.bilibililivestreamchat,
            ),
            calc_leaderboard(
                &mut self.adventures_farm,
            ),
            calc_leaderboard(
                &mut self.casual_pxls,
            ),
            calc_leaderboard(
                &mut self.ironmouse_pxls,
            ),
        );
    }
}

pub async fn send_performances(
    sender: &mut Option<broadcast::Sender<UserChatPerformance>>,
    performances: Vec<UserChatPerformance>,
) {
    if let Some(tx) = sender {
        for performance in performances {
            if let Err(e) = tx.send(performance) {
                error!("Error sending performance to leaderboards: {}", e);
            }
        }

        // after the swap, sender will be dropped, which closes all rx
        std::mem::swap(sender, &mut None);
    }
}
