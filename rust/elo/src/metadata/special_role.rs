/*
Figures out if the user is a special role
*/

use std::collections::HashMap;

use crate::_types::clptypes::{MetadataTypes, MetadataUpdate};
use crate::metadata::metadatatrait::AbstractMetadata;
use twitch_utils::twitchtypes::Comment;
use twitch_utils::TwitchAPIWrapper;

const SPECIAL_ROLES: [&str; 3] = ["moderator", "vip", "broadcaster"];

#[derive(Default, Debug)]
pub struct SpecialRole;

impl AbstractMetadata for SpecialRole {
    /*
    Figures out if the user is a special role
    */

    async fn new(_twitch: &TwitchAPIWrapper) -> Self {
        Self
    }

    fn get_name(&self) -> String {
        "special_role".to_string()
    }

    fn get_default_value(&self) -> MetadataTypes {
        MetadataTypes::Bool(false)
    }

    fn get_metadata(&self, comment: Comment, _sequence_no: u32) -> MetadataUpdate {
        let mut metadata: HashMap<String, MetadataTypes> = HashMap::new();
        let user_badges = comment.message.user_badges;
        if user_badges.is_none() {
            metadata.insert(comment.commenter._id.clone(), MetadataTypes::Bool(false));
            return MetadataUpdate {
                metadata_name: self.get_name(),
                updates: metadata,
            };
        }
        let user_badges = user_badges.unwrap();
        for badge in user_badges {
            if SPECIAL_ROLES.contains(&badge._id.as_str()) {
                metadata.insert(comment.commenter._id.clone(), MetadataTypes::Bool(true));
                return MetadataUpdate {
                    metadata_name: self.get_name(),
                    updates: metadata,
                };
            }
        }
        metadata.insert(comment.commenter._id.clone(), MetadataTypes::Bool(false));
        MetadataUpdate {
            metadata_name: self.get_name(),
            updates: metadata,
        }
    }
}
