/*
Contains all the Twitch types parsable from the chat log
*/

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ChatMessageFragmentEmoticon {
    /* 
    Represents an emoticon in a chat message fragment.
    */
    pub emoticon_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ChatMessageFragment {
    /*
    Represents a fragment of a chat message.
    */
    pub text: String,
    pub emoticon: Option<ChatMessageFragmentEmoticon>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Badge {
    /*
    Represents a badge.
    */
    pub _id: String,
    pub version: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ChatMessage {
    /*
    Represents a chat message.
    */
    pub body: String,
    pub bits_spent: Option<u32>,
    pub fragments: Vec<ChatMessageFragment>,
    pub user_badges: Option<Vec<Badge>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ChatUserInfo {
    /*
    Represents a user in a chat.
    */
    pub display_name: String,
    pub _id: String,
    pub logo: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Comment {
    /*
    Represents a comment in a chat.
    */
    pub _id: String,
    pub message: ChatMessage,
    pub commenter: ChatUserInfo,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ChatLog {
    /*
    Represents a chat log.
    */
    pub comments: Vec<Comment>,
}