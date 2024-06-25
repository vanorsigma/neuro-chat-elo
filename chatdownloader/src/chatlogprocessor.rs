use std::{env, fs, collections::HashMap};
use std::time::Instant;
use log::debug;
use serde_json;

use crate::twitch_utils::TwitchAPIWrapper;
use crate::_types::twitchtypes::{ChatLog, Comment};
use crate::_types::clptypes::UserChatPerformance;

struct ChatLogProcessor {
    twitch: TwitchAPIWrapper,
}

impl ChatLogProcessor {
    fn new(twitch: TwitchAPIWrapper) -> Self {
        Self { twitch }
    }
}