use std::collections::HashSet;

use lazy_static::lazy_static;
use log::{debug, info};

use crate::twitchtypes::{ChatMessageFragment, Comment, SevenTVEmote, TwitchEmote};

const VED_CH_ID: &str = "85498365";

lazy_static! {
    static ref SEVEN_TV_URL: String = format!("https://7tv.io/v3/users/twitch/{}", VED_CH_ID);
}

#[derive(Default)]
pub struct SevenTVClient {
    seventv_emotes: Vec<SevenTVEmote>,
    seventv_lookup: HashSet<String>,
}

impl SevenTVClient {
    pub async fn new() -> Self {
        info!("Getting the 7TV channel emotes");
        let response = reqwest::get(SEVEN_TV_URL.clone()).await;
        if response.is_err() {
            info!("Cannot get 7tv emotes");
            return Self {
                seventv_emotes: Vec::new(),
                seventv_lookup: HashSet::new(),
            };
        }

        let resp_body: serde_json::Value = response.unwrap().json().await.unwrap();
        let mut ret_val = Vec::new();
        if let Some(raw_emotes) = resp_body["emote_set"]["emotes"].as_array() {
            for raw_emote in raw_emotes {
                let host_url = raw_emote["data"]["host"]["url"].as_str().unwrap();
                let filename = raw_emote["data"]["host"]["files"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|emote| emote["name"].as_str().unwrap().ends_with(".webp"))
                    .max_by_key(|emote| emote["width"].as_i64().unwrap())
                    .unwrap();
                ret_val.push(SevenTVEmote {
                    id: raw_emote["id"].as_str().unwrap().to_owned(),
                    name: raw_emote["name"].as_str().unwrap().to_owned(),
                    emote_url: format!("https:{}/{}", host_url, filename["name"].as_str().unwrap()),
                });
            }
        } else {
            info!("Cannot access the required keys to get the emotes");
        }

        debug!("Got {} 7tv emotes", ret_val.len());
        let seventv_lookup: HashSet<String> =
            ret_val.iter().map(|emote| emote.name.clone()).collect();
        Self {
            seventv_emotes: ret_val,
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
        let mut emotes: Vec<TwitchEmote> = Vec::new();
        if let Some(emote) = &fragment.emoticon {
            emotes.push(TwitchEmote::from_twitch_name_id(
                fragment.text.clone(),
                emote.emoticon_id.clone(),
            ));
        }
        emotes.extend(
            self.get_7tv_emotes_in_fragment(fragment)
                .iter()
                .map(|emote| TwitchEmote::from(emote.clone())),
        );
        emotes
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
