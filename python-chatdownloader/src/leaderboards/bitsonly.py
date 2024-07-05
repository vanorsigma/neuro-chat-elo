"""
Bits leaderboard
"""

from typing import Optional

from _types import UserChatPerformance

from .abstractleaderboard import AbstractLeaderboard


class BitsOnly(AbstractLeaderboard):
    """
    Leaderboard for bits-only
    """
    @classmethod
    def get_name(cls):
        return 'bits-only'

    def calculate_score(self,
                        performance: UserChatPerformance) -> Optional[float]:
        return performance.metrics['bits'] if 'bits' in performance.metrics \
            else 0
