/*
Contains all the Twitch types parsable from the chat log
*/

use serde::{Deserialize, Serialize};
use twitch_api::helix::chat::{ChannelEmote, GlobalEmote};
use crate::seventvtypes::RawSevenTVEmote;

const TWITCH_EMOTE_URL: &str = "https://static-cdn.jtvnw.net/emoticons/v2";
const TWITCH_EMOTE_URL_ENDING: &str = "default/light/1.0";

#[derive(Debug, Clone)]
pub struct TwitchEmote {
    pub id: String,
    pub name: String,
    pub url: String,
}

impl TwitchEmote {
    pub fn new(id: String, name: String, url: String) -> Self {
        Self { id, name, url }
    }

    /// Helper function for creating a TwitchEmote from a Twitch emote name and id.
    ///
    /// Used for extracting emotes from a ChatLog without contacting the API
    pub fn from_twitch_name_id(name: String, id: String) -> Self {
        Self {
            id: id.clone(),
            name,
            url: format!("{}/{}/{}", TWITCH_EMOTE_URL, id, TWITCH_EMOTE_URL_ENDING),
        }
    }
}

impl From<SevenTVEmote> for TwitchEmote {
    fn from(seventv_emote: SevenTVEmote) -> Self {
        Self {
            id: seventv_emote.id,
            name: seventv_emote.name,
            url: seventv_emote.emote_url,
        }
    }
}

impl From<GlobalEmote> for TwitchEmote {
    fn from(global_emote: GlobalEmote) -> Self {
        Self {
            id: global_emote.id.to_string(),
            name: global_emote.name,
            url: global_emote.images.url_4x,
        }
    }
}

impl From<ChannelEmote> for TwitchEmote {
    fn from(channel_emote: ChannelEmote) -> Self {
        Self {
            id: channel_emote.id.to_string(),
            name: channel_emote.name,
            url: channel_emote.images.url_4x,
        }
    }
}

impl From<RawSevenTVEmote> for SevenTVEmote {
    fn from(raw_emote: RawSevenTVEmote) -> Self {
        let largest_width_file = raw_emote.host.files.iter().max_by_key(|file| file.width);
        if let Some(file) = largest_width_file {
            let url = raw_emote.host.url + "/" + &file.name;
            Self {
                id: raw_emote.id,
                name: raw_emote.name,
                emote_url: url,
            }
        } else {
            // Use technical difficulties emote if no files are found
            Self {
                id: raw_emote.id,
                name: raw_emote.name,
                emote_url: String::from("https://cdn.7tv.app/emote/63384017cf7eb48c4e731a79/4x.webp"),
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct SevenTVEmote {
    pub id: String,
    pub name: String,
    pub emote_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChatMessageFragmentEmoticon {
    /*
    Represents an emoticon in a chat message fragment.
    */
    pub emoticon_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChatMessageFragment {
    /*
    Represents a fragment of a chat message.
    */
    pub text: String,
    pub emoticon: Option<ChatMessageFragmentEmoticon>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Badge {
    /*
    Represents a badge.
    */
    pub _id: String,
    pub version: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChatMessage {
    /*
    Represents a chat message.
    */
    pub body: String,
    pub bits_spent: u32,
    pub fragments: Vec<ChatMessageFragment>,
    pub user_badges: Option<Vec<Badge>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChatUserInfo {
    /*
    Represents a user in a chat.
    */
    pub display_name: String,
    pub _id: String,
    pub logo: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Comment {
    /*
    Represents a comment in a chat.
    */
    pub _id: String,
    pub message: ChatMessage,
    pub commenter: ChatUserInfo,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChatLog {
    /*
    Represents a chat log.
    */
    pub comments: Vec<Comment>,
}