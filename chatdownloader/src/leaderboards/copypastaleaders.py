"""
Copypasta leaders leaderboard
"""

from typing import Optional

from _types import UserChatPerformance

from .abstractleaderboard import AbstractLeaderboard


class CopypastaLeaders(AbstractLeaderboard):
    """
    Leaderboard for copypasta leaderboard
    """
    @classmethod
    def get_name(cls):
        return 'copypasta'

    def calculate_score(self,
                        performance: UserChatPerformance) -> Optional[float]:
        return sum(metric['copypasta'] for metric in
                   performance.metrics.values()
                   if 'copypasta' in performance.metrics.values())
