use discord_utils::DiscordMessage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use twitch_utils::twitchtypes::Comment;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserChatPerformance {
    pub id: String,
    pub username: String,
    pub avatar: String,
    pub metrics: HashMap<String, f32>,
    pub metadata: HashMap<String, MetadataTypes>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BadgeInformation {
    pub description: String,
    pub image_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum MetadataTypes {
    Bool(bool),
    BadgeList(Vec<BadgeInformation>),
    BasicInfo(String, String),
}

impl MetadataTypes {
    pub fn get_badge_list(&self) -> Option<&Vec<BadgeInformation>> {
        match self {
            MetadataTypes::BadgeList(badge_list) => Some(badge_list),
            _ => None,
        }
    }
    pub fn get_bool(&self) -> Option<&bool> {
        match self {
            MetadataTypes::Bool(b) => Some(b),
            _ => None,
        }
    }
    pub fn get_basic_info(&self) -> Option<(String, String)> {
        match self {
            MetadataTypes::BasicInfo(username, avatar) => {
                Some((username.to_string(), avatar.to_string()))
            }
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct MetricUpdate {
    pub metric_name: String,
    pub updates: HashMap<String, f32>,
}

#[derive(Default)]
pub struct MetadataUpdate {
    pub metadata_name: String,
    pub updates: HashMap<String, MetadataTypes>,
}

impl MetricUpdate {
    pub fn empty_with_name(name: String) -> Self {
        Self {
            metric_name: name,
            updates: HashMap::new(),
        }
    }
}

impl MetadataUpdate {
    pub fn empty_with_name(name: String) -> Self {
        Self {
            metadata_name: name,
            updates: HashMap::new(),
        }
    }
}

/// Message enum representing all possible messages that go through
/// chat log processor.
#[derive(Debug, Clone)]
pub enum Message {
    /// Represents a Twitch message
    Twitch(Comment),
    /// Represents a Discord message
    Discord(DiscordMessage)
}

impl From<Comment> for Message {
    fn from(value: Comment) -> Self {
        Self::Twitch(value)
    }
}
