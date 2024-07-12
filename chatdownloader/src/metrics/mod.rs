pub mod bits;
pub mod copypastaleader;
pub mod emote;
pub mod metrictrait;
pub mod subs;
pub mod text;

use log::warn;
use std::collections::HashMap;
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use crate::_types::twitchtypes::Comment;
use crate::metrics::metrictrait::AbstractMetric;

fn spawn_thread<M>(
    mut metric: M,
    sender: mpsc::Sender<(String, HashMap<String, f32>)>,
    mut reciever: broadcast::Receiver<(Comment, u32)>,
) -> JoinHandle<()>
where
    M: AbstractMetric + Sync + Send + 'static,
{
    /*
    Spawn a thread to update the metrics based on chat messages sent by a tokio broadcast channel
    */
    tokio::task::spawn(async move {
        loop {
            let (comment, sequence_no) = match reciever.recv().await {
                Ok((comment, sequence_no)) => (comment, sequence_no),
                Err(_) => break,
            };
            let metric_result = metric.get_metric(comment, sequence_no);
            match sender.send(metric_result).await {
                Ok(_) => {}
                Err(_) => warn!("Failed to send metric result"),
            };
        }
        let metric_result = metric.finish();
        match sender.send(metric_result).await {
            Ok(_) => {}
            Err(_) => warn!("Failed to send final metric result"),
        };
    })
}

/// Spawn threads to process metrics
/// Returns a vector of join handles, a sender to send messages to the threads, and a receiver to recieve messages from the threads
pub async fn get_metrics() -> (
    HashMap<String, f32>,
    Vec<JoinHandle<()>>,
    broadcast::Sender<(Comment, u32)>,
    mpsc::Receiver<(String, HashMap<String, f32>)>,
) {
    /*
    Spawn threads to process metrics
    Returns a vector of join handles, a sender to send messages to the threads, and a receiver to recieve messages from the threads
    */
    let mut metrics: HashMap<String, f32> = HashMap::new();
    let mut handles = vec![];
    let (broadcast_sender, broadcast_receiver) = broadcast::channel(100000);
    let (mpsc_sender, mpsc_receiver) = mpsc::channel(100000);

    // Initialize the metrics
    let bits = bits::Bits::new().await;
    let subs = subs::Subs::new().await;
    let text = text::Text::new().await;
    let copypastaleader = copypastaleader::CopypastaLeader::new().await;
    let emote = emote::Emote::new().await;

    // Add names and default values to the metrics
    metrics.insert(bits.get_name(), 0.0);
    metrics.insert(subs.get_name(), 0.0);
    metrics.insert(text.get_name(), 0.0);
    metrics.insert(copypastaleader.get_name(), 0.0);
    metrics.insert(emote.get_name(), 0.0);

    // Spawn threads for each metric
    handles.push(spawn_thread(
        bits,
        mpsc_sender.clone(),
        broadcast_receiver.resubscribe(),
    ));
    handles.push(spawn_thread(
        subs,
        mpsc_sender.clone(),
        broadcast_receiver.resubscribe(),
    ));
    handles.push(spawn_thread(
        text,
        mpsc_sender.clone(),
        broadcast_receiver.resubscribe(),
    ));
    handles.push(spawn_thread(
        copypastaleader,
        mpsc_sender.clone(),
        broadcast_receiver.resubscribe(),
    ));
    handles.push(spawn_thread(
        emote,
        mpsc_sender.clone(),
        broadcast_receiver.resubscribe(),
    ));

    (metrics, handles, broadcast_sender, mpsc_receiver)
}
