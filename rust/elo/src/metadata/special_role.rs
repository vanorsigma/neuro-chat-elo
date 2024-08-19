use std::collections::HashMap;

use crate::_types::clptypes::{Message, MetadataTypes, MetadataUpdate};
use crate::metadata::metadatatrait::AbstractMetadata;
use discord_utils::DiscordMessage;
use twitch_utils::twitchtypes::Comment;
use twitch_utils::TwitchAPIWrapper;

const SPECIAL_ROLES_TWITCH: [&str; 3] = ["moderator", "vip", "broadcaster"];
const SPECIAL_ROLES_DISCORD: [&str; 3] = ["Admin", "Moderator", "Twitch Mod"];

/// Figures out if the user is a special role
#[derive(Default, Debug)]
pub struct SpecialRole;

impl SpecialRole {
    fn get_metadata_twitch(&self, comment: Comment) -> MetadataUpdate {
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
            if SPECIAL_ROLES_TWITCH.contains(&badge._id.as_str()) {
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

    fn get_metadata_discord(&self, msg: DiscordMessage) -> MetadataUpdate {
        MetadataUpdate {
            metadata_name: self.get_name(),
            updates: msg
                .author
                .roles
                .iter()
                .rfind(|role| SPECIAL_ROLES_DISCORD.contains(&role.name.as_str()))
                .map(|_| HashMap::from([(msg.author.id, MetadataTypes::Bool(true))]))
                .unwrap_or_default(),
        }
    }
}

impl AbstractMetadata for SpecialRole {
    async fn new(_twitch: &TwitchAPIWrapper) -> Self {
        Self
    }

    fn get_name(&self) -> String {
        "special_role".to_string()
    }

    fn get_default_value(&self) -> MetadataTypes {
        MetadataTypes::Bool(false)
    }

    fn get_metadata(&self, message: Message, _sequence_no: u32) -> MetadataUpdate {
        match message {
            Message::Twitch(comment) => self.get_metadata_twitch(comment),
            Message::Discord(msg) => self.get_metadata_discord(msg),
            _ => MetadataUpdate::default(),
        }
    }
}
