//! Get the username and avatar of the user
use std::collections::HashMap;
use std::sync::Arc;

use crate::_types::clptypes::{Message, MetadataTypes, MetadataUpdate};
use crate::_types::{CASUAL_NEURO_FACTION, IRONMOUSE_NEURO_FACTION};
use crate::metadata::metadatatrait::AbstractMetadata;
use discord_utils::DiscordClient;
use twitch_utils::seventvclient::SevenTVClient;
use twitch_utils::TwitchAPIWrapper;

/// Figures out if the user is a special role
pub struct BasicInfo {
    seventv_client: Arc<SevenTVClient>,
    twitch_client: Arc<TwitchAPIWrapper>,
    discord_client: Arc<DiscordClient>,
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
            Message::Pxls(user) => {
                if let (Some(CASUAL_NEURO_FACTION), Some(discord_tag)) =
                    (user.faction, user.discord_tag)
                {
                    MetadataUpdate {
                        metadata_name: self.get_name(),
                        updates: self
                            .discord_client
                            .get_username_author(discord_tag)
                            .await
                            .map(|author| {
                                HashMap::from([(
                                    author.id,
                                    MetadataTypes::BasicInfo(author.nickname, author.avatar_url),
                                )])
                            })
                            .unwrap_or(HashMap::new()),
                    }
                } else {
                    MetadataUpdate::default()
                }
            }
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
    pub fn new(
        seventv_client: Arc<SevenTVClient>,
        twitch_client: Arc<TwitchAPIWrapper>,
        discord_client: Arc<DiscordClient>,
    ) -> Self {
        Self {
            seventv_client,
            twitch_client,
            discord_client,
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
