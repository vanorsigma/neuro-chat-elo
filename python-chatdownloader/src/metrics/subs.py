"""
The subs metrics
"""
import re

from _types import Comment, ChatMessageFragment

from .abstractmetric import AbstractMetric

WEIGHT_SUBS = 0.02

GIFTED_SUB_REGEX_1 = re.compile(
    r"(?P<gifter>[a-zA-Z0-9_]+) gifted a Tier (?P<tier>[0-9]) Sub to (?P<receiver>[a-zA-Z0-9_]+)!"
)
GIFTED_SUB_REGEX_2 = re.compile(
    r"(?P<gifter>[a-zA-Z0-9_]+) is gifting (?P<no_of_subs>[0-9]+) Tier (?P<tier>[0-9]) Subs to (?P<streamer>[a-zA-Z0-9_]+)\'s community!"
)


class Subs(AbstractMetric):
    """
    The subs metric.
    """
    @staticmethod
    def _no_of_gifted_subs(message: ChatMessageFragment) -> int:
        matches_1 = GIFTED_SUB_REGEX_1.match(message.text)
        matches_2 = GIFTED_SUB_REGEX_2.match(message.text)

        total = 0
        if matches_1 is not None:
            total += 1
        if matches_2 is not None:
            total += int(matches_2.group("no_of_subs"))
        return total

    @classmethod
    def can_parallelize(cls) -> bool:
        return True

    @classmethod
    def get_name(cls) -> str:
        return 'subs'

    def get_metric(self, comment: Comment, sequence_no: int
                   ) -> dict[str, float]:
        return self._shortcut_for_this_comment_user(
            comment,
            sum(self._no_of_gifted_subs(fragment)
                for fragment in comment.message.fragments) * WEIGHT_SUBS)
