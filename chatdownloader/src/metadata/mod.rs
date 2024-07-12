pub mod badges;
pub mod basic_info;
pub mod metadatatrait;
pub mod special_role;

use log::warn;
use std::collections::HashMap;
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use crate::_types::clptypes::MetadataTypes;
use crate::_types::twitchtypes::Comment;
use crate::metadata::metadatatrait::AbstractMetadata;
use crate::twitch_utils::TwitchAPIWrapper;

fn spawn_thread<M>(
    metric: M,
    sender: mpsc::Sender<(String, HashMap<String, MetadataTypes>)>,
    mut reciever: broadcast::Receiver<(Comment, u32)>,
) -> JoinHandle<()>
where
    M: AbstractMetadata + Sync + Send + 'static,
{
    /*
    Spawn a thread to update the metadata based on chat messages sent by a tokio broadcast channel
    */
    tokio::task::spawn(async move {
        loop {
            let (comment, sequence_no) = match reciever.recv().await {
                Ok((comment, sequence_no)) => (comment, sequence_no),
                Err(_) => break,
            };
            let metadata = metric.get_metadata(comment, sequence_no);
            match sender.send(metadata).await {
                Ok(_) => {}
                Err(_) => warn!("Failed to send metadata result"),
            };
        }
    })
}

/// Spawn threads to process metadata
/// Returns a vector of join handles, a sender to send messages to the threads, and a receiver to recieve messages from the threads
pub async fn get_metadata(
    twitch: &TwitchAPIWrapper,
) -> (
    HashMap<String, MetadataTypes>,
    Vec<JoinHandle<()>>,
    broadcast::Sender<(Comment, u32)>,
    mpsc::Receiver<(String, HashMap<String, MetadataTypes>)>,
) {
    /*
    Spawn threads to process metadata
    Returns a vector of join handles, a sender to send messages to the threads, and a receiver to recieve messages from the threads
    */
    let mut metadata: HashMap<String, MetadataTypes> = HashMap::new();
    let mut handles = vec![];
    let (broadcast_sender, broadcast_receiver) = broadcast::channel(100000);
    let (mpsc_sender, mpsc_receiver) = mpsc::channel(100000);

    // Initialize the metadata
    let basic_info = basic_info::BasicInfo::new(twitch).await;
    let badges = badges::Badges::new(twitch).await;
    let special_role = special_role::SpecialRole::new(twitch).await;

    // Add names and default values to the metadata
    metadata.insert(basic_info.get_name(), basic_info.get_default_value());
    metadata.insert(badges.get_name(), badges.get_default_value());
    metadata.insert(special_role.get_name(), special_role.get_default_value());

    // Spawn threads for each metadata
    handles.push(spawn_thread(
        basic_info,
        mpsc_sender.clone(),
        broadcast_receiver.resubscribe(),
    ));
    handles.push(spawn_thread(
        badges,
        mpsc_sender.clone(),
        broadcast_receiver.resubscribe(),
    ));
    handles.push(spawn_thread(
        special_role,
        mpsc_sender.clone(),
        broadcast_receiver.resubscribe(),
    ));

    (metadata, handles, broadcast_sender, mpsc_receiver)
}
