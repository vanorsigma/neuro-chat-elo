"""
Abstract Leaderboard
"""

from abc import ABC
from typing import abstractmethod
from _types import UserChatPerformance


class AbstractLeaderboard(ABC):
    """
    A Leaderboard tells the program how to export a leaderboard
    """
    def __init__(self):
        self.read_initial_state()

    @abstractmethod
    def read_initial_state(self):
        """
        Reads the initial state of a leaderboard, and return a
        leaderboard object ready to digest information.
        """

    @abstractmethod
    def update_leaderboard(self, performance: UserChatPerformance) -> None:
        """
        Updates the current leaderboard with the performance stated in
        the 'performance' variable

        :param: performance: An individual unit of performance
        """

    @abstractmethod
    def save(self) -> None:
        """
        Saves the leaderboard.
        """
