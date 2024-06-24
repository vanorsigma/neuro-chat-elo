/*
The subs metric
*/
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

use crate::_types::twitchtypes::{Comment, ChatMessageFragment};
use crate::metrics::metrictrait::AbstractMetric;

const WEIGHT_SUBS: f32 = 0.1;

lazy_static! {
    static ref GIFTED_SUB_REGEX_1: Regex = Regex::new(
        r"(?P<gifter>[a-zA-Z0-9_]+) gifted a Tier (?P<tier>[0-9]) Sub to (?P<receiver>[a-zA-Z0-9_]+)!"
    ).unwrap();
    static ref GIFTED_SUB_REGEX_2: Regex = Regex::new(
        r"(?P<gifter>[a-zA-Z0-9_]+) is gifting (?P<no_of_subs>[0-9]+) Tier (?P<tier>[0-9]) Subs to (?P<streamer>[a-zA-Z0-9_]+)'s community!"
    ).unwrap();
}

#[allow(dead_code)]
struct Subs;

impl AbstractMetric for Subs {
    fn can_parallelize() -> bool {
        true
    }

    fn get_name() -> String {
        String::from("subs")
    }

    fn get_metric(&mut self, comment: &Comment, _sequence_no: u32) -> HashMap<String, f32> {
        let total_subs: i32 = comment.message.fragments.iter()
            .map(|fragment| Self::no_of_gifted_subs(fragment))
            .sum();

        let score = total_subs as f32 * WEIGHT_SUBS;
        self._shortcut_for_this_comment_user(comment, score)
    }
}

impl Subs {
    fn no_of_gifted_subs(message: &ChatMessageFragment) -> i32 {
        let mut total = 0;

        if let Some(caps) = GIFTED_SUB_REGEX_1.captures(&message.text) {
            total += 1;
        }

        if let Some(caps) = GIFTED_SUB_REGEX_2.captures(&message.text) {
            if let Ok(num) = caps.name("no_of_subs").unwrap().as_str().parse::<i32>() {
                total += num;
            }
        }

        total
    }
}