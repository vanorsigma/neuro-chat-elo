"""
The overall leaderboard
"""

import json
import logging
import os
from dataclasses import dataclass

from _types import LeaderboardExportItem, UserChatPerformance

from .abstractleaderboard import AbstractLeaderboard

DEFAULT_ELO = 1200
K = 0.5


class Overall(AbstractLeaderboard):
    """
    Computes the overall leaderboard (Special + Non-Special).
    """
    def __init__(self):
        self.state = {}
        super().__init__()

    @classmethod
    def get_name(cls):
        return 'overall'

    def calculate_score(self, performance: UserChatPerformance) -> float:
        return sum(performance.metrics.values())
