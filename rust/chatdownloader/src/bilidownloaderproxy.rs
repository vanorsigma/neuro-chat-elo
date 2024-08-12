///! Tries to get the latest Bilibili VOD, and downloads the associated chat
use std::path::Path;

use bili_utils::BiliChatMessage;
use elo::_types::clptypes::Message;

pub struct BiliChatDownloader {
    // user_id: u64,
}

impl BiliChatDownloader {
    pub fn new(/*user_id: u64*/) -> Self {
        Self { /*user_id*/ }
    }

    async fn download_chat(self) {
        todo!("Bili chat hasn't been implemented yet")
    }

    pub fn from_path(self, path: &Path) -> Vec<BiliChatMessage> {
        serde_json::de::from_reader(std::fs::File::open(path).unwrap()).unwrap()
    }
}
