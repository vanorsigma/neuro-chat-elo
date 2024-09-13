//! The subs metric
use lazy_static::lazy_static;
use regex::Regex;

use crate::_types::clptypes::{Message, MetricUpdate};
use crate::metrics::metrictrait::AbstractMetric;
use twitch_utils::twitchtypes::ChatMessageFragment;

const WEIGHT_SUBS: f32 = 0.1;

lazy_static! {
    static ref GIFTED_SUB_REGEX_1: Regex = Regex::new(
        r"(?P<gifter>[a-zA-Z0-9_]+) gifted a Tier (?P<tier>[0-9]) Sub to (?P<receiver>[a-zA-Z0-9_]+)!"
    ).unwrap();
    static ref GIFTED_SUB_REGEX_2: Regex = Regex::new(
        r"(?P<gifter>[a-zA-Z0-9_]+) is gifting (?P<no_of_subs>[0-9]+) Tier (?P<tier>[0-9]) Subs to (?P<streamer>[a-zA-Z0-9_]+)'s community!"
    ).unwrap();
}

#[derive(Default, Debug)]
pub struct Subs;

impl Subs {
    pub fn new() -> Self {
        Self {}
    }
}

impl AbstractMetric for Subs {
    fn can_parallelize(&self) -> bool {
        true
    }

    fn get_name(&self) -> String {
        String::from("subs")
    }

    async fn get_metric(&mut self, message: Message, _sequence_no: u32) -> MetricUpdate {
        match message {
            Message::Twitch(comment) => {
                let total_subs: i32 = comment
                    .message
                    .fragments
                    .iter()
                    .map(no_of_gifted_subs)
                    .sum();

                let score = total_subs as f32 * WEIGHT_SUBS;
                self.twitch_comment_shortcut(comment, score)
            }
            _ => MetricUpdate::empty_with_name(self.get_name()),
        }
    }
}

fn no_of_gifted_subs(message: &ChatMessageFragment) -> i32 {
    let mut total = 0;

    if let Some(_caps) = GIFTED_SUB_REGEX_1.captures(&message.text) {
        total += 1;
    }

    if let Some(caps) = GIFTED_SUB_REGEX_2.captures(&message.text) {
        if let Ok(num) = caps.name("no_of_subs").unwrap().as_str().parse::<i32>() {
            total += num;
        }
    }

    total
}
