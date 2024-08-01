use crate::_types::clptypes::{Message, MetricUpdate};
/*
The text metric
*/
use crate::metrics::metrictrait::AbstractMetric;

const WEIGHT_TEXT: f32 = 0.02;

#[derive(Default, Debug)]
pub struct Text;

impl AbstractMetric for Text {
    async fn new() -> Self {
        Self
    }

    fn can_parallelize(&self) -> bool {
        false
    }

    fn get_name(&self) -> String {
        String::from("text")
    }

    fn get_metric(&mut self, message: Message, _sequence_no: u32) -> MetricUpdate {
        match message {
            Message::TWITCH(comment) => {
                let score = f32::max(0.0, calculate_score(comment.message.body.len()));
                self._shortcut_for_this_comment_user(comment, score)
            },
        }
    }
}

fn calculate_score(x: usize) -> f32 {
    -WEIGHT_TEXT * x as f32 * (x as f32 - 20.0)
}
