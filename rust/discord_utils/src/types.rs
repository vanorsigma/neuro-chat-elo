use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Discord "API" responses
#[derive(Deserialize)]
pub struct DiscordProfileUserResponse {
    pub(crate) id: String,
    pub(crate) avatar: String,
    pub(crate) global_name: String,
}

#[derive(Deserialize)]
pub struct DiscordProfileResponse {
    pub(crate) user: DiscordProfileUserResponse,
}

/// Discord Chat CLI responses

#[derive(Clone, Deserialize, Debug)]
pub struct DiscordRole {
    pub id: String,
    pub name: String,
    pub position: u32,
}

#[derive(Clone, Deserialize, Debug)]
pub struct DiscordAuthor {
    pub id: String,
    pub name: String,
    pub nickname: String,
    pub roles: Vec<DiscordRole>,
    #[serde(alias = "avatarUrl")]
    pub avatar_url: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct DiscordMessage {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub content: String,
    pub author: DiscordAuthor,
}

#[derive(Clone, Deserialize, Debug)]
pub struct DiscordChatLogs {
    pub messages: Vec<DiscordMessage>,
}

#[derive(Clone, Debug)]
pub struct DiscordMapping {
    pub id: String,
    pub name: String,
    pub image_url: String,
}
