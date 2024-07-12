mod bitsonly;
mod chatonly;
mod copypastaleaders;
mod leaderboardtrait;
mod nonvips;
mod overall;
mod subsonly;

use tokio::{sync::broadcast, task::JoinHandle};

use crate::{_types::clptypes::UserChatPerformance, leaderboards::leaderboardtrait::AbstractLeaderboard};

fn spawn_thread<M>(
    mut leaderboard: M,
    mut reciever: broadcast::Receiver<UserChatPerformance>,
) -> JoinHandle<()>
where
    M: AbstractLeaderboard + Sync + Send + 'static,
{
    /*
    Spawn a thread to update the metadata based on chat messages sent by a tokio broadcast channel
    */
    tokio::task::spawn(async move {
        loop {
            let user_chat_performance: UserChatPerformance = match reciever.recv().await {
                Ok(user_chat_performance) => user_chat_performance,
                Err(_) => break,
            };
            leaderboard.update_leaderboard(user_chat_performance);
        }
        leaderboard.save()
    })
}

/// Spawn threads to process metadata
/// Returns a vector of join handles, a sender to send messages to the threads, and a receiver to recieve messages from the threads
pub fn get_leaderboards() -> (
    Vec<JoinHandle<()>>,
    broadcast::Sender<UserChatPerformance>,
) {
    /*
    Spawn threads to process leaderboard
    Returns a vector of join handles and a sender to send messages to the threads
    */
    let mut handles = vec![];
    let (broadcast_sender, broadcast_receiver) = broadcast::channel(100000);

    
    // Initialize the leaderboards
    let bitsonly = bitsonly::BitsOnly::new();
    let chatonly = chatonly::ChatOnly::new();
    let copypasta = copypastaleaders::CopypastaLeaders::new();
    let nonvips = nonvips::NonVIPS::new();
    let overall = overall::Overall::new();
    let subsonly = subsonly::SubsOnly::new();

    // Spawn threads for each metadata
    handles.push(spawn_thread(
        bitsonly,
        broadcast_receiver.resubscribe(),
    ));
    handles.push(spawn_thread(
        chatonly,
        broadcast_receiver.resubscribe(),
    ));
    handles.push(spawn_thread(
        copypasta,
        broadcast_receiver.resubscribe(),
    ));
    handles.push(spawn_thread(
        nonvips,
        broadcast_receiver.resubscribe(),
    ));
    handles.push(spawn_thread(
        overall,
        broadcast_receiver.resubscribe(),
    ));
    handles.push(spawn_thread(
        subsonly,
        broadcast_receiver.resubscribe(),
    ));

    (handles, broadcast_sender)
}
