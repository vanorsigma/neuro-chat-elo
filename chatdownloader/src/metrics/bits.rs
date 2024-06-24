use crate::_types::twitchtypes::Comment;
use crate::metrics::metrictrait::AbstractMetric;

use std::collections::HashMap;

const WEIGHT_BITS: f32 = 0.1;

#[allow(dead_code)]
struct Bits;

impl AbstractMetric for Bits {
    fn can_parallelize() -> bool {
        true
    }

    fn get_name() -> String {
        String::from("bits")
    }

    fn get_metric(&self, comment: &Comment, _sequence_no: u32) -> HashMap<String, f32> {
        let score = comment.message.bits_spent.unwrap_or(0) as f32 * WEIGHT_BITS;
        self._shortcut_for_this_comment_user(comment, score)
    }
}
