use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SevenTVResponse {
    pub emote_set: SevenTVEmoteSet,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SevenTVEmoteSet {
    pub emotes: SevenTVEmotesBundle,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SevenTVEmotesBundle {
    pub data: Vec<RawSevenTVEmote>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawSevenTVEmote {
    pub id: String,
    pub name: String,
    pub host: SevenTVEmoteHost,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SevenTVEmoteHost {
    pub url: String,
    pub files: Vec<SevenTVEmoteFile>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SevenTVEmoteFile {
    pub name: String,
    pub static_name: String,
    pub width: i64,
}