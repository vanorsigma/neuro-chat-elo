use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct DiscordRole {
    pub id: String,
    pub name: String,
    pub position: u32
}

#[derive(Clone, Deserialize, Debug)]
pub struct DiscordAuthor {
    pub id: String,
    pub name: String,
    pub nickname: String,
    pub roles: Vec<DiscordRole>,
    #[serde(alias = "avatarUrl")]
    pub avatar_url: String
}

#[derive(Clone, Deserialize, Debug)]
pub struct DiscordMessage {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub content: String,
    pub author: DiscordAuthor
}

#[derive(Clone, Deserialize, Debug)]
pub struct DiscordChatLogs {
    pub messages: Vec<DiscordMessage>,
}

#[derive(Clone, Debug)]
pub struct DiscordMapping {
    pub id: String,
    pub name: String,
    pub image_url: String
}
