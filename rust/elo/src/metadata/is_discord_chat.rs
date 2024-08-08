//! Tags the update as a discord chat-related change
use std::collections::HashMap;

use crate::_types::clptypes::{Message, MetadataTypes, MetadataUpdate};
use crate::metadata::metadatatrait::AbstractMetadata;
use twitch_utils::TwitchAPIWrapper;

/// Figures out if the chat is associated to discord
#[derive(Default, Debug)]
pub struct IsDiscordChat;

impl AbstractMetadata for IsDiscordChat {
    async fn new(_twitch: &TwitchAPIWrapper) -> Self {
        Self
    }

    fn get_name(&self) -> String {
        "is_discord_chat".to_string()
    }

    fn get_default_value(&self) -> MetadataTypes {
        MetadataTypes::Bool(false)
    }

    fn get_metadata(&self, message: Message, _sequence_no: u32) -> MetadataUpdate {
        match message {
            Message::Discord(msg) => MetadataUpdate {
                metadata_name: self.get_name(),
                updates: HashMap::from([(
                    msg.author.id,
                    MetadataTypes::Bool(true)
                )]),
            },
            _ => MetadataUpdate::empty_with_name(self.get_name())
        }
    }
}
