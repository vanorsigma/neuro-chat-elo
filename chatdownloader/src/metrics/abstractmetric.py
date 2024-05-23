"""
Provides the AbstractMethod class
"""
from _types import Comment

from abc import ABC, abstractmethod


class AbstractMetric(ABC):
    """
    Abstract class for a metric.
    """

    def _shortcut_for_this_comment_user(
            self, comment: Comment, score: int) -> list[dict[str, int]]:
        return {comment.commenter._id: score}

    @classmethod
    @abstractmethod
    def get_name(cls) -> str:
        """
        Name of the metric
        """

    @abstractmethod
    def get_metric(self, comment: Comment,
                   sequence_no: int) -> dict[str, int]:
        """
        Gets the score for a particular comment

        :param comment: The comment to process
        :param sequence_no: The sequence number of the comment
        :return: A dict. The dict contains the user id and
                 the score to add for the user involved in this
                 metric.
        """

    def finish(self) -> dict[str, int]:
        """
        This method is called when there are no more comments to process.
        Useful for metrics that need to flush any remaining data.

        :return: A dict. The dict contains the user id and
                 the score to add for the user involved in this
                 metric.
        """
        return []
