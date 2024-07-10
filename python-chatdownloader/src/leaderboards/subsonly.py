"""
Subs leaderboard
"""

from typing import Optional

from _types import UserChatPerformance

from .abstractleaderboard import AbstractLeaderboard


class SubsOnly(AbstractLeaderboard):
    """
    Leaderboard for subs only
    """
    @classmethod
    def get_name(cls):
        return 'subs-only'

    def calculate_score(self,
                        performance: UserChatPerformance) -> Optional[float]:
        return performance.metrics['subs'] if 'subs' in performance.metrics \
            else 0
