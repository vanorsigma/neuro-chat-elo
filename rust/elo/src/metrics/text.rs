//! The text metric
use std::collections::HashMap;

use crate::_types::clptypes::{Message, MetricUpdate};
use crate::metrics::metrictrait::AbstractMetric;

const WEIGHT_TEXT: f32 = 0.02;

#[derive(Default, Debug)]
pub struct Text;

impl Text {
    pub fn new() -> Self {
        Self {}
    }
}

impl AbstractMetric for Text {
    fn can_parallelize(&self) -> bool {
        false
    }

    fn get_name(&self) -> String {
        String::from("text")
    }

    fn get_metric(&mut self, message: Message, _sequence_no: u32) -> MetricUpdate {
        match message {
            Message::Twitch(comment) => {
                let score = f32::max(0.0, calculate_score(comment.message.body.len()));
                self.twitch_comment_shortcut(comment, score)
            }
            Message::Discord(msg) => MetricUpdate {
                metric_name: self.get_name(),
                updates: HashMap::from([(
                    msg.author.id,
                    f32::max(0.0, calculate_score(msg.content.len())),
                )]),
            },
            _ => MetricUpdate::default(),
        }
    }
}

fn calculate_score(x: usize) -> f32 {
    -WEIGHT_TEXT * x as f32 * (x as f32 - 20.0)
}
