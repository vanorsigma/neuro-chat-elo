"""
The non-VIPs leaderboard
"""

from typing import Optional

from _types import UserChatPerformance

from .abstractleaderboard import AbstractLeaderboard


class NonVIPS(AbstractLeaderboard):
    """
    Leaderboard for non-VIP users
    """
    @classmethod
    def get_name(cls):
        return 'nonvips'

    def calculate_score(self,
                        performance: UserChatPerformance) -> Optional[float]:
        if 'special_role' not in performance.metadata \
           and not performance.metadata['special_role']:
            return None
        return sum(performance.metrics.values())
