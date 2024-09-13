//! Get the username and avatar of the user
use std::collections::HashMap;
use std::sync::Arc;

use crate::_types::clptypes::{Message, MetadataTypes, MetadataUpdate};
use crate::metadata::metadatatrait::AbstractMetadata;
use twitch_utils::seventvclient::SevenTVClient;
use twitch_utils::TwitchAPIWrapper;

const IRONMOUSE_NEURO_FACTION: u64 = 1;

/// Figures out if the user is a special role
pub struct BasicInfo {
    seventv_client: Arc<SevenTVClient>,
    twitch_client: Arc<TwitchAPIWrapper>,
}

impl AbstractMetadata for BasicInfo {
    fn get_name(&self) -> String {
        "basic_info".to_string()
    }

    fn get_default_value(&self) -> MetadataTypes {
        MetadataTypes::BasicInfo("".to_string(), "".to_string())
    }

    async fn get_metadata(&self, message: Message, _sequence_no: u32) -> MetadataUpdate {
        match message {
            Message::Twitch(comment) => self.process_twitch(comment),
            Message::Discord(msg) => MetadataUpdate {
                metadata_name: self.get_name(),
                updates: HashMap::from([(
                    msg.author.id,
                    MetadataTypes::BasicInfo(msg.author.nickname, msg.author.avatar_url),
                )]),
            },
            Message::Bilibili(msg) => MetadataUpdate {
                metadata_name: self.get_name(),
                updates: HashMap::from([(
                    msg.uid,
                    MetadataTypes::BasicInfo(msg.username, msg.avatar),
                )]),
            },
            Message::Adventures(rank) => MetadataUpdate {
                metadata_name: self.get_name(),
                updates: HashMap::from([(
                    rank.uid,
                    MetadataTypes::BasicInfo(rank.user, rank.avatar),
                )]),
            },
            Message::Pxls(user) => MetadataUpdate {
                metadata_name: self.get_name(),
                updates: HashMap::from([(
                    "DISCORD-".to_string()
                        + user
                            .discord_tag
                            .as_ref()
                            .unwrap_or(&"!@#$%(&)DISCORD".to_string()), // scuffed ignore string
                    MetadataTypes::BasicInfo(
                        user.discord_tag.unwrap_or("".to_string()),
                        "".to_string(),
                    ),
                )]),
            },
            Message::IronmousePixels(user) => {
                if let Some(IRONMOUSE_NEURO_FACTION) = user.faction {
                    MetadataUpdate {
                        metadata_name: self.get_name(),
                        updates: self
                            .twitch_client
                            .get_user_from_username(user.pxls_username.clone())
                            .await
                            .map(|user| {
                                HashMap::from([(
                                    user._id,
                                    MetadataTypes::BasicInfo(user.display_name, user.logo),
                                )])
                            })
                            .unwrap_or(HashMap::new()),
                    }
                } else {
                    MetadataUpdate::default()
                }
            }
            _ => MetadataUpdate::default(),
        }
    }
}

impl BasicInfo {
    pub fn new(seventv_client: Arc<SevenTVClient>, twitch_client: Arc<TwitchAPIWrapper>) -> Self {
        Self {
            seventv_client,
            twitch_client,
        }
    }

    fn process_twitch(&self, comment: twitch_utils::twitchtypes::Comment) -> MetadataUpdate {
        MetadataUpdate {
            metadata_name: self.get_name(),
            updates: self
                .seventv_client
                .get_emotes_in_comment(&comment)
                .into_iter()
                .map(|emote| (emote.id, MetadataTypes::BasicInfo(emote.name, emote.url)))
                .chain(std::iter::once((
                    comment.commenter._id,
                    MetadataTypes::BasicInfo(
                        comment.commenter.display_name,
                        comment.commenter.logo,
                    ),
                )))
                .collect(),
        }
    }
}
