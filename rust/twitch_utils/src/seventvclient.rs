use std::{collections::HashSet, vec};
use crate::seventvtypes::SevenTVResponse;

use log::{debug, info};

use crate::twitchtypes::{ChatMessageFragment, Comment, SevenTVEmote, TwitchEmote};

const SEVEN_TV_URL: &str = "https://7tv.io/v3/users/twitch/85498365";

#[derive(Default)]
pub struct SevenTVClient {
    seventv_emotes: Vec<SevenTVEmote>,
    seventv_lookup: HashSet<String>,
}

impl SevenTVClient {
    pub async fn new() -> Self {
        info!("Getting the 7TV channel emotes");
        let response = reqwest::get(SEVEN_TV_URL).await;
        if response.is_err() {
            info!("Cannot get 7tv emotes");
            return Self {
                seventv_emotes: Vec::new(),
                seventv_lookup: HashSet::new(),
            };
        }

        let response: SevenTVResponse = response.unwrap().json::<SevenTVResponse>().await.unwrap();
        let seventv_emotes: Vec<SevenTVEmote> = response.emote_set
            .emotes
            .data
            .iter()
            .map(|emote| SevenTVEmote::from(emote.clone()))
            .collect();
        
        let seventv_lookup: HashSet<String> = seventv_emotes
            .iter()
            .map(|emote| emote.name.clone())
            .collect();

        Self {
            seventv_emotes,
            seventv_lookup,
        }
    }

    pub fn get_7tv_emotes_in_fragment(&self, fragment: &ChatMessageFragment) -> Vec<SevenTVEmote> {
        fragment
            .text
            .split(' ')
            .filter(|word| self.seventv_lookup.contains(*word))
            .map(|word| {
                self.seventv_emotes
                    .iter()
                    .find(|e| e.name == word)
                    .unwrap()
                    .clone()
            })
            .collect()
    }

    pub fn get_emotes_in_fragment(&self, fragment: &ChatMessageFragment) -> Vec<TwitchEmote> {
        self.get_7tv_emotes_in_fragment(fragment)
            .iter()
            .map(|emote| TwitchEmote::from(emote.clone()))
            .chain(fragment.emoticon.iter().map(|s| {
                TwitchEmote::from_twitch_name_id(fragment.text.clone(), s.emoticon_id.clone())
            }))
            .collect()
    }

    pub fn get_emotes_in_comment(&self, comment: &Comment) -> Vec<TwitchEmote> {
        let emotes: Vec<TwitchEmote> = comment
            .message
            .fragments
            .iter()
            .flat_map(|fragment| self.get_emotes_in_fragment(fragment))
            .collect();
        debug!(
            "Got {:?} emotes in comment: {}",
            emotes.len(),
            comment.message.body
        );
        emotes
    }
}
