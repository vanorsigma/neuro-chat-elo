use crate::_types::clptypes::{Message, MetricUpdate};
use crate::metrics::metrictrait::AbstractMetric;

const WEIGHT_BITS: f32 = 0.1;

#[derive(Default, Debug)]
pub struct Bits;

impl Bits {
    pub fn new() -> Self {
        Self {}
    }
}

impl AbstractMetric for Bits {
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
                self.twitch_comment_shortcut(comment, score)
            }
            _ => MetricUpdate::empty_with_name(self.get_name()),
        }
    }
}
