use crate::_types::twitchtypes::Comment;
use std::collections::HashMap;

pub trait AbstractMetric {
    /*
    Defines the trait for a metric
    */
    async fn new() -> Self where Self: Sized;
        /*
        Initializes the metric
        */

    fn _shortcut_for_this_comment_user(&self, comment: Comment, score: f32) -> HashMap<String, f32> {
        // return {comment.commenter._id: score}
        let mut map: HashMap<String, f32> = HashMap::new();
        map.insert(comment.commenter._id.clone(), score);
        map
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

    fn get_metric(&mut self, comment: Comment, sequence_no: u32) -> HashMap<String, f32>;
        /*
        Gets the score for a particular comment

        :param comment: The comment to process
        :param sequence_no: The sequence number of the comment
        :return: A HashMap. The HashMap contains the user id and
                 the score to add for the user involved in this
                 metric.
        */

    fn finish(&self) -> HashMap<String, f32> {
        /*
        This method is called when there are no more comments to process.
        Useful for metrics that need to flush any remaining data.

        :return: A HashMap. The HashMap contains the user id and
                 the score to add for the user involved in this
                 metric.
        */
        HashMap::new()
    }
}
