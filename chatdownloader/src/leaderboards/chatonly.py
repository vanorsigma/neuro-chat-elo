"""
Chat-only leaderboard
"""

from typing import Optional

from _types import UserChatPerformance

from .abstractleaderboard import AbstractLeaderboard


class ChatOnly(AbstractLeaderboard):
    """
    Leaderboard for chat-only
    """
    @classmethod
    def get_name(cls):
        return 'chat-only'

    def calculate_score(self,
                        performance: UserChatPerformance) -> Optional[float]:
        return performance.metrics['text'] if 'text' in performance.metrics \
            else 0
