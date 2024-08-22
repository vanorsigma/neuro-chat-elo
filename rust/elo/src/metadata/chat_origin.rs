//! Tags the update with a chat origin
use std::collections::HashMap;
use std::sync::Arc;

use crate::_types::clptypes::{Message, MessageTag, MetadataTypes, MetadataUpdate};
use crate::metadata::metadatatrait::AbstractMetadata;
use twitch_utils::seventvclient::SevenTVClient;
use twitch_utils::twitchtypes::Comment;

/// Figures out the association of a message to a chat origin
pub struct ChatOrigin {
    seventv_client: Arc<SevenTVClient>,
}

impl AbstractMetadata for ChatOrigin {
    fn get_name(&self) -> String {
        "chat_origin".to_string()
    }

    fn get_default_value(&self) -> MetadataTypes {
        MetadataTypes::ChatOrigin(MessageTag::None)
    }

    fn get_metadata(&self, message: Message, _sequence_no: u32) -> MetadataUpdate {
        MetadataUpdate {
            metadata_name: self.get_name(),
            updates: match &message {
                Message::Twitch(comment) => self.process_twitch(comment, &message),
                Message::Discord(msg) => HashMap::from([(
                    msg.author.id.to_string(),
                    MetadataTypes::ChatOrigin(MessageTag::from(&message)),
                )]),
                _ => HashMap::new(),
            },
        }
    }
}

impl ChatOrigin {
    pub fn new(seventv_client: Arc<SevenTVClient>) -> Self {
        Self {
            seventv_client,
        }
    }

    pub fn process_twitch(
        &self,
        comment: &Comment,
        message: &Message,
    ) -> HashMap<String, MetadataTypes> {
        self.seventv_client
            .get_emotes_in_comment(comment)
            .iter()
            .map(|emote| {
                (
                    emote.id.clone(),
                    MetadataTypes::ChatOrigin(MessageTag::from(emote)),
                )
            })
            .chain(std::iter::once((
                comment.commenter._id.clone(),
                MetadataTypes::ChatOrigin(MessageTag::from(message)),
            )))
            .collect()
    }
}
