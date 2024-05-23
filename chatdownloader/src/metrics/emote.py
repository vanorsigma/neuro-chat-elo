"""
The emote metrics
"""
from _types import Comment

from .abstractmetric import AbstractMetric

WEIGHT_EMOTES = 0.02


class Emote(AbstractMetric):
    """
    The emote metric.

    TODO: 7tv support
    """

    @classmethod
    def get_name(cls) -> str:
        return 'emote'

    def get_metric(self, comment: Comment, sequence_no: int) -> int:
        return self._shortcut_for_this_comment_user(
            comment,
            sum(int(fragment.emoticon is None)
                for fragment in comment.message.fragments) * WEIGHT_EMOTES)
