/*
Assigns badges to each user
*/
use log::error;
use std::collections::HashMap;

use crate::_types::twitchtypes::Comment;
use crate::_types::clptypes::{BadgeInformation, MetadataTypes};
use crate::_constants::VED_CH_ID;
use crate::twitch_utils::TwitchAPIWrapper;
use crate::metadata::metadatatrait::AbstractMetadata;

pub struct Badges {
    badges: HashMap<String, HashMap<String, BadgeInformation>>
}

impl AbstractMetadata for Badges {
    async fn new(twitch: TwitchAPIWrapper) -> Self {
        let badges = twitch.get_badges(VED_CH_ID.to_string()).await.unwrap();
        Self {
            badges
        }
    }

    fn get_name(&self) -> String {
        "Badges".to_string()
    }

    fn get_default_value(&self) -> MetadataTypes {
        MetadataTypes::BadgeList(vec![])
    }

    fn get_metadata(&self, comment: Comment, _sequence_no: u32) -> HashMap<String, MetadataTypes> {
        let mut metadata: Vec<BadgeInformation> = vec![];
        let user_badges = comment.message.user_badges;
        if user_badges.is_none() {
            let mut out: HashMap<String, MetadataTypes> = HashMap::new();
            out.insert(comment.commenter._id, MetadataTypes::BadgeList(vec![]));
            return out;
        }
        let user_badges = user_badges.unwrap();
        for badge in user_badges {
            let badge_set = self.badges.get(&badge._id);
            if badge_set.is_none() {
                error!("Badge set not found for badge id {}", badge._id);
                continue;
            }
            let badge_set = badge_set.unwrap();
            let badge_info = badge_set.get(&badge.version);
            if badge_info.is_none() {
                error!("Badge info not found for badge id {} and version {}", badge._id, badge.version);
                continue;
            }
            metadata.push(
                badge_info.unwrap()
                .clone()
            );
        };
        HashMap::from([(comment.commenter._id.clone(), MetadataTypes::BadgeList(metadata))])
    }
}