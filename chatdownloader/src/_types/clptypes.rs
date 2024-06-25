use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct UserChatPerformance {
    pub id: String,
    pub username: String,
    pub avatar: String,
    pub metrics: HashMap<String, f32>,
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BadgeInformation {
    pub description: String,
    pub image_url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Metric<T>{
    pub commenter_id: String,
    pub value: T,
}