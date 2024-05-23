"""
The text metric
"""
from _types import Comment

from .abstractmetric import AbstractMetric

WEIGHT_TEXT = 0.01


class Text(AbstractMetric):
    """
    Text for a metric
    """
    @classmethod
    def get_name(cls) -> str:
        return 'text'

    def get_metric(self, comment: Comment, sequence_no: int) -> int:
        return self._shortcut_for_this_comment_user(
            comment,
            sum(len(fragment.text)
                for fragment in comment.message.fragments) * WEIGHT_TEXT)
