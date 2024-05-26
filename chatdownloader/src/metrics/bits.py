"""
The bits metrics
"""
from _types import Comment

from .abstractmetric import AbstractMetric

WEIGHT_BITS = 0.02


class Bits(AbstractMetric):
    """
    Bits metric
    """

    @classmethod
    def can_parallelize(cls) -> bool:
        return True

    @classmethod
    def get_name(cls) -> str:
        return 'bits'

    def get_metric(self, comment: Comment, sequence_no: int \
                   ) -> list[dict[str, int]]:
        return self._shortcut_for_this_comment_user(
            comment, comment.message.bits_spent * WEIGHT_BITS)
