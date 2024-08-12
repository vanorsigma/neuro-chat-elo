use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct BiliChatMessage {
    pub username: String,
    pub message: String,
    pub uid: String,
    pub avatar: String,
}
