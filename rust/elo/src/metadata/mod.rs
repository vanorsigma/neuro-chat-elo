pub mod badges;
pub mod basic_info;
pub mod chat_origin;
pub mod metadatatrait;
pub mod special_role;

use discord_utils::DiscordClient;
use futures::join;
use log::debug;
use log::warn;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use twitch_utils::seventvclient::SevenTVClient;

use crate::_types::clptypes::Message;
use crate::_types::clptypes::MetadataTypes;
use crate::_types::clptypes::MetadataUpdate;
use crate::metadata::metadatatrait::AbstractMetadata;
use twitch_utils::TwitchAPIWrapper;

pub struct MetadataProcessor {
    pub defaults: HashMap<String, MetadataTypes>,
    broadcast_receiver: broadcast::Receiver<(Message, u32)>,
    mpsc_sender: mpsc::Sender<MetadataUpdate>,
    basic_info: basic_info::BasicInfo,
    badges: badges::Badges,
    special_role: special_role::SpecialRole,
    chat_origin: chat_origin::ChatOrigin,
}

impl MetadataProcessor {
    pub async fn new(
        twitch: Arc<TwitchAPIWrapper>,
        seventv_client: Arc<SevenTVClient>,
        discord_client: Arc<DiscordClient>,
        broadcast_receiver: broadcast::Receiver<(Message, u32)>,
        mpsc_sender: mpsc::Sender<MetadataUpdate>,
    ) -> Self {
        let mut defaults: HashMap<String, MetadataTypes> = HashMap::new();

        // Initialize the metadata
        let basic_info = basic_info::BasicInfo::new(seventv_client.clone(), twitch.clone(), discord_client.clone());
        let badges = badges::Badges::new(&twitch).await;
        let special_role = special_role::SpecialRole::new();
        let chat_origin = chat_origin::ChatOrigin::new(seventv_client, twitch, discord_client);

        // Add names and default values to the metadata
        defaults.insert(basic_info.get_name(), basic_info.get_default_value());
        defaults.insert(badges.get_name(), badges.get_default_value());
        defaults.insert(special_role.get_name(), special_role.get_default_value());
        defaults.insert(chat_origin.get_name(), chat_origin.get_default_value());

        Self {
            defaults,
            broadcast_receiver,
            mpsc_sender,
            basic_info,
            badges,
            special_role,
            chat_origin,
        }
    }

    pub async fn run(&mut self) {
        join!(
            calc_metadata(
                &mut self.basic_info,
                self.mpsc_sender.clone(),
                self.broadcast_receiver.resubscribe(),
            ),
            calc_metadata(
                &mut self.badges,
                self.mpsc_sender.clone(),
                self.broadcast_receiver.resubscribe(),
            ),
            calc_metadata(
                &mut self.special_role,
                self.mpsc_sender.clone(),
                self.broadcast_receiver.resubscribe(),
            ),
            calc_metadata(
                &mut self.chat_origin,
                self.mpsc_sender.clone(),
                self.broadcast_receiver.resubscribe(),
            ),
        );
        debug!("All metadata finished");
    }
}

async fn calc_metadata<M: AbstractMetadata + Send + Sync + 'static>(
    metadata: &mut M,
    sender: mpsc::Sender<MetadataUpdate>,
    mut reciever: broadcast::Receiver<(Message, u32)>,
) {
    /*
    Find metadata based on chat messages sent by a tokio broadcast channel
    */
    while let Ok((message, sequence_no)) = reciever.recv().await {
        let metadata = (*metadata).get_metadata(message, sequence_no).await;
        if let Err(e) = sender.send(metadata).await {
            warn!("Failed to send metadata result {}", e)
        };
    }
}

/// Get the default values for the metrics and set up the channels
pub async fn setup_metadata_and_channels(
    twitch: Arc<TwitchAPIWrapper>,
    seventv_client: Arc<SevenTVClient>,
    discord_client: Arc<DiscordClient>
) -> (
    MetadataProcessor,
    broadcast::Sender<(Message, u32)>,
    mpsc::Receiver<MetadataUpdate>,
) {
    let (broadcast_sender, broadcast_receiver) = broadcast::channel(100000);
    let (mpsc_sender, mpsc_receiver) = mpsc::channel(100000);
    let metadata_processor =
        MetadataProcessor::new(twitch, seventv_client, discord_client, broadcast_receiver, mpsc_sender).await;
    (metadata_processor, broadcast_sender, mpsc_receiver)
}
