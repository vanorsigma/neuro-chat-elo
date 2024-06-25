/*
Assigns badges to each user
*/
use log::error;
use std::collections::HashMap;

use crate::_types::twitchtypes::Comment;
use crate::_types::clptypes::BadgeInformation;
use crate::_constants::VED_CH_ID;
use crate::twitch_utils::TwitchAPIWrapper;
use crate::metadata::metadatatrait::AbstractMetadata;

struct Badges {
    twitch: TwitchAPIWrapper,
    badges: HashMap<String, HashMap<String, BadgeInformation>>
}

impl AbstractMetadata for Badges {
    type MetadataType = Vec<BadgeInformation>;
    
    #[tokio::main]
    async fn new(twitch: TwitchAPIWrapper) -> Self {
        let badges = twitch.get_badges(VED_CH_ID.to_string()).await.unwrap();
        Self {
            twitch,
            badges
        }
    }

    fn get_name() -> String {
        "Badges".to_string()
    }

    fn get_metadata(&self, comment: Comment, sequence_no: u32) -> HashMap<String, Self::MetadataType> {
        let mut metadata: Vec<BadgeInformation> = vec![];
        let user_badges = comment.message.user_badges;
        if user_badges.is_none() {
            let mut out: HashMap<String, Self::MetadataType> = HashMap::new();
            out.insert(comment.commenter._id, vec![]);
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
        let mut metadata_map: HashMap<String, Vec<BadgeInformation>> = HashMap::new();
        metadata_map.insert(comment.commenter._id, metadata);
        metadata_map
    }
}