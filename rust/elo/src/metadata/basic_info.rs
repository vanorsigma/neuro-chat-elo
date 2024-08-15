//! Get the username and avatar of the user
use std::collections::HashMap;

use crate::_types::clptypes::{Message, MetadataTypes, MetadataUpdate};
use crate::metadata::metadatatrait::AbstractMetadata;
use twitch_utils::TwitchAPIWrapper;

/// Figures out if the user is a special role
#[derive(Default, Debug)]
pub struct BasicInfo;

impl AbstractMetadata for BasicInfo {
    async fn new(_twitch: &TwitchAPIWrapper) -> Self {
        Self
    }

    fn get_name(&self) -> String {
        "basic_info".to_string()
    }

    fn get_default_value(&self) -> MetadataTypes {
        MetadataTypes::BasicInfo("".to_string(), "".to_string())
    }

    fn get_metadata(&self, message: Message, _sequence_no: u32) -> MetadataUpdate {
        match message {
            Message::Twitch(comment) => MetadataUpdate {
                metadata_name: self.get_name(),
                updates: HashMap::from([(
                    comment.commenter._id.clone(),
                    MetadataTypes::BasicInfo(
                        comment.commenter.display_name.clone(),
                        comment.commenter.logo.clone(),
                    ),
                )]),
            },
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
