/*
The emote metric
*/

use std::collections::{HashMap, HashSet};

use log::{info, debug};
use serde::Deserialize;
use lazy_static::lazy_static;

use crate::_types::twitchtypes::{ChatMessageFragment, Comment};
use crate::_constants::VED_CH_ID;

use super::metrictrait::AbstractMetric;

const WEIGHT_EMOTES: f32 = 0.02;

lazy_static! {
    static ref SEVEN_TV_URL: String = format!("https://7tv.io/v3/users/twitch/{}", VED_CH_ID);
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct SevenTVEmote {
    name: String,
    emote_url: String,
}

#[allow(dead_code)]
pub struct Emote {
    seventv_emotes: Vec<SevenTVEmote>,
    seventv_lookup: HashSet<String>,
}

impl Emote {
    fn count_7tv_emotes_in_fragment(&self, fragment: &ChatMessageFragment) -> usize {
        let mut count = 0;
        for word in fragment.text.split(' ') {
            if self.seventv_lookup.contains(word) {
                count += 1;
            }
        }
        debug!("Found {} number of 7TV emotes in {}", count, fragment.text);
        count
    }
}

impl AbstractMetric for Emote {
    async fn new() -> Self {
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
                    name: raw_emote["name"].as_str().unwrap().to_owned(),
                    emote_url: format!("https://{}/{}", host_url, filename["name"]),
                });
            }
        } else {
            info!("Cannot access the required keys to get the emotes");
        }

        debug!("Got {} 7tv emotes", ret_val.len());
        let seventv_lookup: HashSet<String> = ret_val.iter().map(|emote| emote.name.clone()).collect();
        Self {
            seventv_emotes: ret_val,
            seventv_lookup,
        }
    }

    fn can_parallelize(&self) -> bool {
        false
    }

    fn get_name(&self) -> String {
        String::from("emote")
    }

    fn get_metric(&mut self, comment: Comment, _sequence_no: u32) -> HashMap<String, f32> {
        let metric = comment
            .message
            .fragments
            .iter()
            .map(|fragment| {
                (fragment.emoticon.is_some() as u16 as f32
                    + self.count_7tv_emotes_in_fragment(fragment) as f32)
                    * WEIGHT_EMOTES
            })
            .sum();
        let mut result = HashMap::new();
        result.insert("emote".to_owned(), metric);
        result
    }
}