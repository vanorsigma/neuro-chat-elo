//! Get the username and avatar of the user
use std::collections::HashMap;
use std::sync::Arc;

use crate::_types::clptypes::{Message, MetadataTypes, MetadataUpdate};
use crate::metadata::metadatatrait::AbstractMetadata;
use twitch_utils::seventvclient::SevenTVClient;

/// Figures out if the user is a special role
#[derive(Default)]
pub struct BasicInfo {
    seventv_client: Arc<SevenTVClient>,
}

impl AbstractMetadata for BasicInfo {
    fn get_name(&self) -> String {
        "basic_info".to_string()
    }

    fn get_default_value(&self) -> MetadataTypes {
        MetadataTypes::BasicInfo("".to_string(), "".to_string())
    }

    fn get_metadata(&self, message: Message, _sequence_no: u32) -> MetadataUpdate {
        match message {
            Message::Twitch(comment) => self.process_twitch(comment),
            Message::Discord(msg) => MetadataUpdate {
                metadata_name: self.get_name(),
                updates: HashMap::from([(
                    msg.author.id,
                    MetadataTypes::BasicInfo(msg.author.nickname, msg.author.avatar_url),
                )]),
            },
            _ => MetadataUpdate::default(),
        }
    }
}

impl BasicInfo {
    pub fn new(seventv_client: Arc<SevenTVClient>) -> Self {
        Self {
            seventv_client,
        }
    }

    fn process_twitch(&self, comment: twitch_utils::twitchtypes::Comment) -> MetadataUpdate {
        MetadataUpdate {
            metadata_name: self.get_name(),
            updates: self.seventv_client
                .get_emotes_in_comment(&comment)
                .iter()
                .map(|emote| {
                    (
                        emote.id.clone(),
                        MetadataTypes::BasicInfo(emote.name.clone(), emote.url.clone()),
                    )
                })
                .chain(std::iter::once((
                    comment.commenter._id.clone(),
                    MetadataTypes::BasicInfo(
                        comment.commenter.display_name.clone(),
                        comment.commenter.logo.clone(),
                    ),
                )))
                .collect(),
        }
    }
}
