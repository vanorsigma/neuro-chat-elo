//! Tags the update with a chat origin
use std::collections::HashMap;

use crate::_types::clptypes::{Message, MessageTag, MetadataTypes, MetadataUpdate};
use crate::metadata::metadatatrait::AbstractMetadata;
use twitch_utils::TwitchAPIWrapper;

/// Figures out the association of a message to a chat origin
#[derive(Default, Debug)]
pub struct ChatOrigin;

impl AbstractMetadata for ChatOrigin {
    async fn new(_twitch: &TwitchAPIWrapper) -> Self {
        Self
    }

    fn get_name(&self) -> String {
        "chat_origin".to_string()
    }

    fn get_default_value(&self) -> MetadataTypes {
        MetadataTypes::ChatOrigin(MessageTag::None)
    }

    fn get_metadata(&self, message: Message, _sequence_no: u32) -> MetadataUpdate {
        MetadataUpdate {
            metadata_name: self.get_name(),
            updates: HashMap::from([(
                match &message {
                    Message::Discord(msg) => &msg.author.id,
                    Message::Twitch(comment) => &comment.commenter._id,
                    Message::Bilibili(msg) => &msg.uid,
                    _ => return MetadataUpdate::default(),
                }
                .to_string(),
                MetadataTypes::ChatOrigin(MessageTag::from(&message)),
            )]),
        }
    }
}
