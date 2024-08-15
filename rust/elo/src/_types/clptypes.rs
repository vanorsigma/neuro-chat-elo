use discord_utils::DiscordMessage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use twitch_utils::twitchtypes::Comment;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum MetadataTypes {
    Bool(bool),
    BadgeList(Vec<BadgeInformation>),
    BasicInfo(String, String),
    ChatOrigin(MessageTag),
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
macro_rules! declare_messages {
    ($(($variant:ident, $raw_message:ty)),*) => {
        /// Message wraps the underlying data type, to be used as a supertype for message processing
        #[derive(Debug, Clone)]
        #[non_exhaustive]
        pub enum Message {
            $(
                $variant($raw_message),
            )*
            None,
        }

        /// MessageTag acts as a "tag" for a already processed message
        #[derive(Debug, Clone)]
        pub enum MessageTag {
            $($variant,)*
            None,
        }

        $(
            impl From<$raw_message> for Message {
                fn from(value: $raw_message) -> Self {
                    Self::$variant(value)
                }
            }
        )*

        impl From<&Message> for MessageTag {
            fn from(message: &Message) -> Self {
                match message {
                    $(Message::$variant(_) => Self::$variant,)*
                    _ => Self::None,
                }
            }
        }
    };
}

declare_messages!((Twitch, Comment), (Discord, DiscordMessage));
