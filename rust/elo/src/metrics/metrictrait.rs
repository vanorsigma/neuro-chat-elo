use crate::_types::clptypes::MetricUpdate;
use std::collections::HashMap;
use twitch_utils::twitchtypes::Comment;

pub trait AbstractMetric {
    /*
    Defines the trait for a metric
    */
    async fn new() -> Self
    where
        Self: Sized;
    /*
    Initializes the metric
    */

    fn _shortcut_for_this_comment_user(&self, comment: Comment, score: f32) -> MetricUpdate {
        // return {comment.commenter._id: score}
        let mut map: HashMap<String, f32> = HashMap::new();
        map.insert(comment.commenter._id.clone(), score);
        MetricUpdate {
            metric_name: self.get_name(),
            updates: map,
        }
    }

    #[allow(dead_code)]
    fn can_parallelize(&self) -> bool;
    /*
    Indicates to the executor if this metric can be parallelized
    */

    fn get_name(&self) -> String;
    /*
    Returns the name of the metric
    */

    fn get_metric(&mut self, comment: Comment, sequence_no: u32) -> MetricUpdate;
    /*
    Gets the score for a particular comment

    :param comment: The comment to process
    :param sequence_no: The sequence number of the comment
    :return: A HashMap. The HashMap contains the user id and
             the score to add for the user involved in this
             metric.
    */

    fn finish(&self) -> MetricUpdate {
        /*
        This method is called when there are no more comments to process.
        Useful for metrics that need to flush any remaining data.

        :return: A HashMap. The HashMap contains the user id and
                 the score to add for the user involved in this
                 metric.
        */
        MetricUpdate {
            metric_name: self.get_name(),
            updates: HashMap::new(),
        }
    }
}
