"""
The overall leaderboard
"""

from typing import Optional

from _types import UserChatPerformance

from .abstractleaderboard import AbstractLeaderboard


class Overall(AbstractLeaderboard):
    """
    Computes the overall leaderboard (Special + Non-Special).
    """

    @classmethod
    def get_name(cls):
        return 'overall'

    def calculate_score(self,
                        performance: UserChatPerformance) -> Optional[float]:
        return sum(performance.metrics.values())
