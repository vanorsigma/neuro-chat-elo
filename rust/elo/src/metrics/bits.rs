use crate::_types::clptypes::{Message, MetricUpdate};
use crate::metrics::metrictrait::AbstractMetric;

const WEIGHT_BITS: f32 = 0.1;

#[derive(Default, Debug)]
pub struct Bits;

impl AbstractMetric for Bits {
    async fn new() -> Self {
        Self
    }

    fn can_parallelize(&self) -> bool {
        true
    }

    fn get_name(&self) -> String {
        String::from("bits")
    }

    fn get_metric(&mut self, message: Message, _sequence_no: u32) -> MetricUpdate {
        match message {
            Message::Twitch(comment) => {
                let score = comment.message.bits_spent as f32 * WEIGHT_BITS;
                self._shortcut_for_this_comment_user(comment, score)
            }
        }
    }
}
