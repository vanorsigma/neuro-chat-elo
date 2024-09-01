//! Assigns badges to each user
use discord_utils::{DiscordMapping, DiscordMessage};
use lazy_static::lazy_static;
use log::error;
use std::collections::HashMap;
use twitch_utils::twitchtypes::Comment;

use crate::_constants::VED_CH_ID;
use crate::_types::{
    clptypes::{Message, MetadataTypes, MetadataUpdate},
    leaderboardtypes::BadgeInformation,
};
use crate::metadata::metadatatrait::AbstractMetadata;
use twitch_utils::TwitchAPIWrapper;

lazy_static! {
    static ref DISCORD_ROLE_MAPPING: HashMap<String, DiscordMapping> = HashMap::from([
    (
            "574720716025626654".to_string(),
            DiscordMapping {
                id: "574720716025626654".to_string(),
                name: "Admin".to_string(),
                image_url: "https://cdn.discordapp.com/role-icons/574720716025626654/fdba9a82d5acd7285cb800c030fb48ef.webp?size=128&quality=lossless".to_string(),
            },
        ),
        (
            "574931772781887488".to_string(),
            DiscordMapping {
                id: "574931772781887488".to_string(),
                name: "Moderator".to_string(),
                image_url: "https://cdn.discordapp.com/role-icons/574931772781887488/409144b2ac07f5868b1341759fd34e17.webp?size=128&quality=lossless".to_string(),
            },
        ),
        (
            "604550016320929792".to_string(),
            DiscordMapping {
                id: "574931772781887488".to_string(),
                name: "Twitch Mod".to_string(),
                image_url: "https://cdn.discordapp.com/role-icons/574931772781887488/409144b2ac07f5868b1341759fd34e17.webp?size=128&quality=lossless".to_string(),
            },
        ),
        (
            "1059341815754530937".to_string(),
            DiscordMapping {
                id: "1059341815754530937".to_string(),
                name: "VIP".to_string(),
                image_url: "https://cdn.discordapp.com/role-icons/1059341815754530937/760a124960bb2fba95741c9e8c921c50.webp?size=128&quality=lossless".to_string(),
            },
        ),
        (
            "1127037809564327946".to_string(),
            DiscordMapping {
                id: "1127037809564327946".to_string(),
                name: "Super Neuro Fans".to_string(),
                image_url: "https://cdn.discordapp.com/role-icons/1127037809564327946/caa9be70d7da2df4c92933927f70df78.webp?size=128&quality=lossless".to_string(),
            },
        )
    ]);
}

pub struct Badges {
    badges: HashMap<String, HashMap<String, BadgeInformation>>,
}

impl Badges {
    fn get_metadata_twitch(&self, comment: Comment) -> MetadataUpdate {
        let mut metadata: Vec<BadgeInformation> = vec![];
        let user_badges = if let Some(user_badges) = comment.message.user_badges {
            user_badges
        } else {
            let mut out: HashMap<String, MetadataTypes> = HashMap::new();
            out.insert(
                comment.commenter._id.clone(),
                MetadataTypes::BadgeList(vec![]),
            );
            return MetadataUpdate {
                metadata_name: self.get_name(),
                updates: out,
            };
        };
        for badge in user_badges {
            let badge_set = self.badges.get(&badge._id);
            if badge_set.is_none() {
                error!("Badge set not found for badge id {}", badge._id);
                continue;
            }
            let badge_set = badge_set.unwrap();
            let badge_info = badge_set.get(&badge.version);
            if badge_info.is_none() {
                error!(
                    "Badge info not found for badge id {} and version {}",
                    badge._id, badge.version
                );
                continue;
            }
            metadata.push(badge_info.unwrap().clone());
        }
        MetadataUpdate {
            metadata_name: self.get_name(),
            updates: HashMap::from([(
                comment.commenter._id.clone(),
                MetadataTypes::BadgeList(metadata),
            )]),
        }
    }

    fn get_metadata_discord(&self, msg: DiscordMessage) -> MetadataUpdate {
        let metadata = msg
            .author
            .roles
            .iter()
            .filter(|item| DISCORD_ROLE_MAPPING.contains_key(&item.id))
            .map(|item| {
                let discord_info = DISCORD_ROLE_MAPPING.get(&item.id).unwrap();
                BadgeInformation {
                    description: discord_info.name.clone(),
                    image_url: discord_info.image_url.clone(),
                }
            })
            .collect();

        MetadataUpdate {
            metadata_name: self.get_name(),
            updates: HashMap::from([(msg.author.id, MetadataTypes::BadgeList(metadata))]),
        }
    }
}

impl AbstractMetadata for Badges {
    async fn new(twitch: &TwitchAPIWrapper) -> Self {
        let badges = twitch
            .get_badges(VED_CH_ID.to_string())
            .await
            .unwrap()
            .into_iter()
            .map(|(set_id, badge_set)| {
                (
                    set_id.clone(),
                    badge_set
                        .into_iter()
                        .map(|(badge_id, badge)| {
                            (
                                badge_id,
                                BadgeInformation {
                                    description: set_id.clone(),
                                    image_url: badge.image_url_4x,
                                },
                            )
                        })
                        .collect(),
                )
            })
            .collect();
        Self { badges }
    }

    fn get_name(&self) -> String {
        "badges".to_string()
    }

    fn get_default_value(&self) -> MetadataTypes {
        MetadataTypes::BadgeList(vec![])
    }

    fn get_metadata(&self, message: Message, _sequence_no: u32) -> MetadataUpdate {
        match message {
            Message::Twitch(comment) => self.get_metadata_twitch(comment),
            Message::Discord(msg) => self.get_metadata_discord(msg),
            Message::Adventures(rank) => MetadataUpdate {
                metadata_name: self.get_name(),
                updates: HashMap::from([(
                    rank.uid,
                    MetadataTypes::BadgeList(vec![BadgeInformation {
                        description: "".to_string(),
                        image_url: rank.badge,
                    }]),
                )]),
            },
            _ => MetadataUpdate::default(),
        }
    }
}
