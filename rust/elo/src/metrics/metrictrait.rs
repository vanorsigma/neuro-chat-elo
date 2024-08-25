use crate::_types::clptypes::{Message, MetricUpdate};
use std::collections::HashMap;
use twitch_utils::twitchtypes::Comment;

/// Defines the trait for a metric
pub trait AbstractMetric {
    fn twitch_comment_shortcut(&self, comment: Comment, score: f32) -> MetricUpdate {
        let mut map: HashMap<String, f32> = HashMap::new();
        map.insert(comment.commenter._id, score);
        MetricUpdate {
            metric_name: self.get_name(),
            updates: map,
        }
    }

    /// Indicates to the executor if this metric can be parallelized
    #[allow(dead_code)]
    fn can_parallelize(&self) -> bool;

    /// Returns the name of the metric
    fn get_name(&self) -> String;

    /// Gets the score for a particular message.
    ///
    /// # Parameters
    /// - `comment`: The comment to process
    /// - `sequence_no`: The sequence number of the message
    ///
    /// # Return value
    /// A metric update to be added for the associated user
    fn get_metric(&mut self, message: Message, sequence_no: u32) -> MetricUpdate;

    /// This method is called when there are no more comments to
    /// process. Useful for metrics that need to flush any remaining
    /// data.
    fn finish(&self) -> MetricUpdate {
        MetricUpdate {
            metric_name: self.get_name(),
            updates: HashMap::new(),
        }
    }
}
