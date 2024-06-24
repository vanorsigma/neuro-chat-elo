use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserChatPerformance {
    pub id: String,
    pub username: String,
    pub avatar: String
}

#[derive(Deserialize, Serialize)]
pub struct BadgeInformation {
    pub description: String,
    pub image_url: String,
}