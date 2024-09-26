pub mod badges;
pub mod basic_info;
pub mod chat_origin;
pub mod metadatatrait;
pub mod special_role;

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

struct WithReceiver<M: AbstractMetadata> {
    pub metadata: M,
    pub receiver: broadcast::Receiver<(Message, u32)>,
    pub sender: mpsc::Sender<MetadataUpdate>,
}

impl<M: AbstractMetadata> WithReceiver<M> {
    fn new(
        metadata: M,
        receiver: &broadcast::Receiver<(Message, u32)>,
        sender: &mpsc::Sender<MetadataUpdate>,
    ) -> Self {
        Self {
            metadata,
            receiver: receiver.resubscribe(),
            sender: sender.clone(),
        }
    }

    fn get_name(&self) -> String {
        self.metadata.get_name()
    }

    fn get_metadata(&mut self, message: Message, sequence_no: u32) -> MetadataUpdate {
        self.metadata.get_metadata(message, sequence_no)
    }

    fn get_default_value(&self) -> MetadataTypes {
        self.metadata.get_default_value()
    }
}

pub struct MetadataProcessor {
    pub defaults: HashMap<String, MetadataTypes>,
    basic_info: WithReceiver<basic_info::BasicInfo>,
    badges: WithReceiver<badges::Badges>,
    special_role: WithReceiver<special_role::SpecialRole>,
    chat_origin: WithReceiver<chat_origin::ChatOrigin>,
}

impl MetadataProcessor {
    pub async fn new(
        twitch: &TwitchAPIWrapper,
        seventv_client: Arc<SevenTVClient>,
        broadcast_receiver: broadcast::Receiver<(Message, u32)>,
        mpsc_sender: mpsc::Sender<MetadataUpdate>,
    ) -> Self {
        let mut defaults: HashMap<String, MetadataTypes> = HashMap::new();

        // Initialize the metadata
        let basic_info = WithReceiver::new(basic_info::BasicInfo::new(seventv_client.clone()), &broadcast_receiver, &mpsc_sender);
        let badges = WithReceiver::new(badges::Badges::new(twitch).await, &broadcast_receiver, &mpsc_sender);
        let special_role = WithReceiver::new(special_role::SpecialRole::new(), &broadcast_receiver, &mpsc_sender);
        let chat_origin = WithReceiver::new(chat_origin::ChatOrigin::new(seventv_client), &broadcast_receiver, &mpsc_sender);

        // Add names and default values to the metadata
        defaults.insert(basic_info.get_name(), basic_info.get_default_value());
        defaults.insert(badges.get_name(), badges.get_default_value());
        defaults.insert(special_role.get_name(), special_role.get_default_value());
        defaults.insert(chat_origin.get_name(), chat_origin.get_default_value());

        Self {
            defaults,
            basic_info,
            badges,
            special_role,
            chat_origin,
        }
    }

    pub async fn run(&mut self) {
        join!(
            calc_metadata(&mut self.basic_info),
            calc_metadata(&mut self.badges),
            calc_metadata(&mut self.special_role),
            calc_metadata(&mut self.chat_origin),
        );
        debug!("All metadata finished");
    }
}

async fn calc_metadata<M: AbstractMetadata + Send + Sync + 'static>(
    metadata: &mut WithReceiver<M>,
) {
    /*
    Find metadata based on chat messages sent by a tokio broadcast channel
    */
    while let Ok((message, sequence_no)) = metadata.receiver.recv().await {
        let metadata_update = (*metadata).get_metadata(message, sequence_no);
        if let Err(e) = metadata.sender.send(metadata_update).await {
            warn!("Failed to send metadata result {}", e)
        };
    }
}

/// Get the default values for the metrics and set up the channels
pub async fn setup_metadata_and_channels(
    twitch: &TwitchAPIWrapper,
    seventv_client: Arc<SevenTVClient>,
) -> (
    MetadataProcessor,
    broadcast::Sender<(Message, u32)>,
    mpsc::Receiver<MetadataUpdate>,
) {
    let (broadcast_sender, broadcast_receiver) = broadcast::channel(100000);
    let (mpsc_sender, mpsc_receiver) = mpsc::channel(100000);
    let metadata_processor =
        MetadataProcessor::new(twitch, seventv_client, broadcast_receiver, mpsc_sender).await;
    (metadata_processor, broadcast_sender, mpsc_receiver)
}
