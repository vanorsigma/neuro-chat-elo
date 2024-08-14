//! Tags the update as a bilibili chat-related change
use std::collections::HashMap;

use crate::_types::clptypes::{Message, MetadataTypes, MetadataUpdate};
use crate::metadata::metadatatrait::AbstractMetadata;
use twitch_utils::TwitchAPIWrapper;

/// Figures out if the chat is associated to bilibili
#[derive(Default, Debug)]
pub struct IsBilibili;

impl AbstractMetadata for IsBilibili {
    async fn new(_twitch: &TwitchAPIWrapper) -> Self {
        Self
    }

    fn get_name(&self) -> String {
        "is_bilibili_chat".to_string()
    }

    fn get_default_value(&self) -> MetadataTypes {
        MetadataTypes::Bool(false)
    }

    fn get_metadata(&self, message: Message, _sequence_no: u32) -> MetadataUpdate {
        match message {
            Message::Bilibili(msg) => MetadataUpdate {
                metadata_name: self.get_name(),
                updates: HashMap::from([(msg.uid, MetadataTypes::Bool(true))]),
            },
            _ => MetadataUpdate::empty_with_name(self.get_name()),
        }
    }
}
