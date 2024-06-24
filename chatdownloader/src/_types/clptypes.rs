use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct UserChatPerformance {
    pub id: String,
    pub username: String,
    pub avatar: String,
    pub metrics: HashMap<String, f32>,
    pub metadata: UserMetadata,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BadgeInformation {
    pub description: String,
    pub image_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserMetadata {
    pub badges: Option<Vec<BadgeInformation>>,
}