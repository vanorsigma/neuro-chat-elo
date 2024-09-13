//! Tags the update with a chat origin
use std::collections::HashMap;
use std::sync::Arc;

use crate::_types::clptypes::{Message, MessageTag, MetadataTypes, MetadataUpdate};
use crate::_types::{CASUAL_NEURO_FACTION, IRONMOUSE_NEURO_FACTION};
use crate::metadata::metadatatrait::AbstractMetadata;
use discord_utils::DiscordClient;
use twitch_utils::seventvclient::SevenTVClient;
use twitch_utils::twitchtypes::Comment;
use twitch_utils::TwitchAPIWrapper;

/// Figures out the association of a message to a chat origin
pub struct ChatOrigin {
    seventv_client: Arc<SevenTVClient>,
    twitch_client: Arc<TwitchAPIWrapper>,
    discord_client: Arc<DiscordClient>,
}

impl AbstractMetadata for ChatOrigin {
    fn get_name(&self) -> String {
        "chat_origin".to_string()
    }

    fn get_default_value(&self) -> MetadataTypes {
        MetadataTypes::ChatOrigin(MessageTag::None)
    }

    async fn get_metadata(&self, message: Message, _sequence_no: u32) -> MetadataUpdate {
        MetadataUpdate {
            metadata_name: self.get_name(),
            updates: match &message {
                Message::Twitch(comment) => self.process_twitch(comment, &message).await,
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
                    if let (Some(CASUAL_NEURO_FACTION), Some(discord_tag)) =
                        (user.faction, user.discord_tag.clone())
                    {
                        log::info!("processing for {discord_tag}");
                        self.discord_client
                            .get_username_author(discord_tag)
                            .await
                            .map(|author| {
                                HashMap::from([(
                                    author.id,
                                    MetadataTypes::ChatOrigin(MessageTag::from(&message)),
                                )])
                            })
                            .unwrap_or(HashMap::new())
                    } else {
                        HashMap::new()
                    }
                }
                Message::IronmousePixels(user) => {
                    if let Some(IRONMOUSE_NEURO_FACTION) = user.faction {
                        self.twitch_client
                            .get_user_from_username(user.pxls_username.clone())
                            .await
                            .map(|user| {
                                HashMap::from([(
                                    user._id,
                                    MetadataTypes::ChatOrigin(MessageTag::from(&message)),
                                )])
                            })
                            .unwrap_or(HashMap::new())
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

    pub async fn process_twitch(
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
