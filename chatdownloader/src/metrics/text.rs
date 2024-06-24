/*
The text metric
*/
use crate::_types::twitchtypes::Comment;
use crate::metrics::metrictrait::AbstractMetric;

use std::collections::HashMap;

const WEIGHT_TEXT: f32 = 0.02;

pub struct Text;

impl AbstractMetric for Text {
    fn can_parallelize() -> bool {
        false
    }

    fn get_name() -> String {
        String::from("text")
    }

    fn get_metric(&mut self, comment: &Comment, _sequence_no: u32) -> HashMap<String, f32> {
        let score = f32::max(0.0, Self::calculate_score(comment.message.body.len()));
        self._shortcut_for_this_comment_user(comment, score)
    }
}

impl Text {
    fn calculate_score(x: usize) -> f32 {
        -WEIGHT_TEXT * x as f32 * (x as f32 - 20.0)
    }
}