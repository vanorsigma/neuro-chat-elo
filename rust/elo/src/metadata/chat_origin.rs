//! Tags the update with a chat origin
use std::collections::HashMap;
use std::sync::Arc;

use crate::_types::clptypes::{Message, MessageTag, MetadataTypes, MetadataUpdate};
use crate::metadata::metadatatrait::AbstractMetadata;
use twitch_utils::seventvclient::SevenTVClient;
use twitch_utils::twitchtypes::Comment;

const CASUAL_NEURO_FACTION: u64 = 3680;
const IRONMOUSE_NEURO_FACTION: u64 = 1;

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
                Message::Bilibili(msg) => HashMap::from([(
                    msg.uid.to_string(),
                    MetadataTypes::ChatOrigin(MessageTag::from(&message)),
                )]),
                Message::Adventures(rank) => HashMap::from([(
                    rank.uid.to_string(),
                    MetadataTypes::ChatOrigin(MessageTag::from(&message)),
                )]),
                Message::Pxls(user) => {
                    if let Some(CASUAL_NEURO_FACTION) = user.faction {
                        HashMap::from([(
                            "DISCORD-".to_string() + &user.pxls_username,
                            MetadataTypes::ChatOrigin(MessageTag::from(&message)),
                        )])
                    } else {
                        HashMap::new()
                    }
                }
                Message::IronmousePixels(user) => {
                    if let Some(IRONMOUSE_NEURO_FACTION) = user.faction {
                        HashMap::from([(
                            "TWITCH-".to_string() + &user.pxls_username,
                            MetadataTypes::ChatOrigin(MessageTag::from(&message)),
                        )])
                    } else {
                        HashMap::new()
                    }
                }
                _ => HashMap::new(),
            },
        }
    }
}

impl ChatOrigin {
    pub fn new(seventv_client: Arc<SevenTVClient>) -> Self {
        Self { seventv_client }
    }

    pub fn process_twitch(
        &self,
        comment: &Comment,
        message: &Message,
    ) -> HashMap<String, MetadataTypes> {
        self.seventv_client
            .get_emotes_in_comment(comment)
            .into_iter()
            .map(|emote| {
                (
                    emote.id.clone(),
                    MetadataTypes::ChatOrigin(MessageTag::from(&Message::from(emote))),
                )
            })
            .chain(std::iter::once((
                comment.commenter._id.clone(),
                MetadataTypes::ChatOrigin(MessageTag::from(message)),
            )))
            .collect()
    }
}
