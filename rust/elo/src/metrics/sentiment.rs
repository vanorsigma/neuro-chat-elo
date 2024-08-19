//! The sentiment metric
use sentiment::analyze;
use std::collections::HashMap;

use crate::_types::clptypes::{Message, MetricUpdate};
use crate::metrics::metrictrait::AbstractMetric;

const WEIGHT_SENTIMENT: f32 = 0.5;

pub struct Sentiment {}

impl AbstractMetric for Sentiment {
    async fn new() -> Self {
        Self {}
    }

    fn can_parallelize(&self) -> bool {
        false
    }

    fn get_name(&self) -> String {
        String::from("sentiment")
    }

    fn get_metric(&mut self, message: Message, _sequence_no: u32) -> MetricUpdate {
        match message {
            Message::Twitch(comment) => {
                let score = self.calculate_score(comment.message.body.clone());
                self.twitch_comment_shortcut(comment, score)
            }
            Message::Discord(msg) => MetricUpdate {
                metric_name: self.get_name(),
                updates: HashMap::from([(
                    msg.author.id,
                    self.calculate_score(msg.content.clone()),
                )]),
            },
            _ => MetricUpdate::default(),
        }
    }
}

impl Sentiment {
    fn calculate_score(&self, text: String) -> f32 {
        let score: f32 = analyze(text).score;
        score * WEIGHT_SENTIMENT
    }
}
