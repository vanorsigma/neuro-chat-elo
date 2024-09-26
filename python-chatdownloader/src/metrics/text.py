"""
The text metric
"""

import math
from _types import Comment

from .abstractmetric import AbstractMetric

WEIGHT_TEXT = 0.02


class Text(AbstractMetric):
    """
    Text for a metric
    """
    @classmethod
    def can_parallelize(cls) -> bool:
        return False

    @classmethod
    def get_name(cls) -> str:
        return 'text'

    @staticmethod
    def _calculate_score(x) -> float:
        return -WEIGHT_TEXT * x * (x - 20)

    def get_metric(self, comment: Comment,
                   sequence_no: int) -> dict[str, float]:
        return self._shortcut_for_this_comment_user(
            comment,
            # we mathed this
            max(0, self._calculate_score(len(comment.message.body)))
        )
