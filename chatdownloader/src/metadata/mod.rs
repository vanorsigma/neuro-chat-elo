pub mod badges;
pub mod basic_info;
pub mod metadatatrait;
pub mod special_role;

use futures::join;
use log::debug;
use log::warn;
use std::collections::HashMap;
use tokio::sync::broadcast;
use tokio::sync::mpsc;

use crate::_types::clptypes::MetadataTypes;
use crate::_types::twitchtypes::Comment;
use crate::metadata::metadatatrait::AbstractMetadata;
use crate::twitch_utils::TwitchAPIWrapper;
pub struct MetadataProcessor {
    metadata_defaults: HashMap<String, MetadataTypes>,
    broadcast_receiver: Option<broadcast::Receiver<(Comment, u32)>>,
    mpsc_sender: Option<mpsc::Sender<(String, HashMap<String, MetadataTypes>)>>,
    basic_info: basic_info::BasicInfo,
    badges: badges::Badges,
    special_role: special_role::SpecialRole,
}

impl MetadataProcessor {
    pub async fn new(twitch: &TwitchAPIWrapper) -> Self {
        let mut metadata_defaults: HashMap<String, MetadataTypes> = HashMap::new();

        // Initialize the metadata
        let basic_info = basic_info::BasicInfo::new(twitch).await;
        let badges = badges::Badges::new(twitch).await;
        let special_role = special_role::SpecialRole::new(twitch).await;

        // Add names and default values to the metadata
        metadata_defaults.insert(basic_info.get_name(), basic_info.get_default_value());
        metadata_defaults.insert(badges.get_name(), badges.get_default_value());
        metadata_defaults.insert(special_role.get_name(), special_role.get_default_value());

        Self {
            metadata_defaults,
            broadcast_receiver: None,
            mpsc_sender: None,
            basic_info,
            badges,
            special_role,
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn get_defaults_and_setup_channels(
        &mut self,
    ) -> (
        HashMap<String, MetadataTypes>,
        broadcast::Sender<(Comment, u32)>,
        mpsc::Receiver<(String, HashMap<String, MetadataTypes>)>,
    ) {
        let (broadcast_sender, broadcast_receiver) = broadcast::channel(100000);
        let (mpsc_sender, mpsc_receiver) = mpsc::channel(100000);
        self.broadcast_receiver.replace(broadcast_receiver);
        self.mpsc_sender.replace(mpsc_sender);
        (
            self.metadata_defaults.clone(),
            broadcast_sender.clone(),
            mpsc_receiver,
        )
    }

    pub async fn run(&mut self) {
        let mpsc_sender = match &self.mpsc_sender {
            Some(sender) => sender,
            None => {
                panic!("No mpsc sender found, did you run get_defaults_and_setup_channels?");
            }
        };
        let broadcast_receiver = match &self.broadcast_receiver {
            Some(receiver) => receiver,
            None => {
                panic!("No broadcast receiver found, did you run get_defaults_and_setup_channels?");
            }
        };
        join!(
            calc_metadata(
                &mut self.basic_info,
                mpsc_sender.clone(),
                broadcast_receiver.resubscribe(),
            ),
            calc_metadata(
                &mut self.badges,
                mpsc_sender.clone(),
                broadcast_receiver.resubscribe(),
            ),
            calc_metadata(
                &mut self.special_role,
                mpsc_sender.clone(),
                broadcast_receiver.resubscribe(),
            ),
        );
        debug!("All metadata finished");
        drop(self.mpsc_sender.take());
    }
}

async fn calc_metadata<M: AbstractMetadata + Send + Sync + 'static>(
    metadata: &mut M,
    sender: mpsc::Sender<(String, HashMap<String, MetadataTypes>)>,
    mut reciever: broadcast::Receiver<(Comment, u32)>,
) {
    /*
    Find metadata based on chat messages sent by a tokio broadcast channel
    */
    loop {
        let (comment, sequence_no) = match reciever.recv().await {
            Ok((comment, sequence_no)) => (comment, sequence_no),
            Err(_) => break,
        };
        let metadata = (*metadata).get_metadata(comment, sequence_no);
        if let Err(e) = sender.send(metadata).await {
            warn!("Failed to send metadata result {}", e)
        };
        tokio::task::yield_now().await;
    }
}
