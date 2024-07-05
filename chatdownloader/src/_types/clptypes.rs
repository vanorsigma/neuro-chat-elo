use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
}