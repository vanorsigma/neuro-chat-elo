"""
Abstract Leaderboard
"""

import logging
import json
import os

from abc import ABC
from typing import abstractmethod
from _types import UserChatPerformance, LeaderboardInnerState, LeaderboardExportItem


K = 0.5


class AbstractLeaderboard(ABC):
    """
    A Leaderboard tells the program how to export a leaderboard
    """
    def __init__(self):
        self.state = {}
        self.read_initial_state()

    @classmethod
    @abstractmethod
    def get_name(cls) -> str:
        """
        Name of the leaderboard
        """

    @abstractmethod
    def calculate_score(self, performance: UserChatPerformance) -> float:
        """
        Calculates the score given the performance
        """

    def read_initial_state(self):
        """
        Reads the initial state of a leaderboard, and return a
        leaderboard object ready to digest information.
        """
        logging.info(f'Loading {self.get_name()} leaderboard...')
        if not os.path.exists(f'{self.get_name()}.json'):
            logging.info(f'{self.get_name()} leaderboard doesn\'t already exist.')
            return

        with open(f'{self.get_name()}.json', 'r', encoding='utf8') as f:
            data = json.load(f)
            items = [
                LeaderboardExportItem.from_dict(item) for item in data]
            for item in items:
                self.state[item.id] = LeaderboardInnerState(
                    id=item.id,
                    username=item.username,
                    avatar=item.avatar,
                    elo=item.elo
                )
        logging.info(f'{self.get_name()} leaderboard loading ok')

    def update_leaderboard(self, performance: UserChatPerformance) -> None:
        """
        Updates the current leaderboard with the performance stated in
        the 'performance' variable

        :param: performance: An individual unit of performance
        """
        if performance.id not in self.state:
            self.state[performance.id] = LeaderboardInnerState(
                id=performance.id,
                username=performance.username,
                avatar=performance.avatar,
            )

        self.state[performance.id].score = self.calculate_score(performance)

    def __calculate_new_elo(self):
        # n^2 algorithm. For every user, make user 1 "fight" user 2
        # returns a dictionary of new elos
        versus_complete = set()
        for inner_state_1 in self.state.values():
            for inner_state_2 in list(self.state.values()):
                if inner_state_1.id == inner_state_2.id:
                    continue

                if (inner_state_1.id, inner_state_2.id) in versus_complete \
                   or (inner_state_2.id, inner_state_1.id) in versus_complete:
                    continue
                p_1 = (1.0 / (1.0 + 10**(
                    (inner_state_1.elo - inner_state_2.elo) / 400)))
                p_2 = (1.0 / (1.0 + 10**(
                    (inner_state_2.elo - inner_state_1.elo) / 400)))

                # slight variant: if tie, we award elo to both
                p1_won = int(inner_state_1.score >= inner_state_2.score)
                p2_won = int(inner_state_2.score >= inner_state_1.score)
                inner_state_1.elo += K * (p1_won - p_1)
                inner_state_2.elo += K * (p2_won - p_2)

                versus_complete.add((inner_state_1.id, inner_state_2.id))

    def save(self) -> None:
        """
        Saves the leaderboard.
        """
        logging.info(f'Saving {self.get_name()} leaderboard...')
        self.__calculate_new_elo()
        values = list(self.state.values())
        to_save = [LeaderboardExportItem(
            id=inner_state.id,
            rank=0,
            elo=inner_state.elo,
            username=inner_state.username,
            delta=0,
            avatar=inner_state.avatar
        ) for inner_state in values]

        # update rank and delta
        to_save.sort(key=lambda x: x.elo, reverse=True)
        assert len(to_save) > 1, 'Nothing to save!'

        to_save[0].rank = 1
        for idx, item in enumerate(to_save[1:]):
            rank = to_save[idx].rank + (int(item.elo < to_save[idx].elo))
            item.delta = ((values[idx].previous_rank - rank) if values[idx].previous_rank else 0)
            item.rank = rank

        with open(f'{self.get_name()}.json', 'w', encoding='utf8') as f:
            logging.info('Now writing to overall leaderboard...')
            json.dump([data.to_dict() for data in to_save], f)

        logging.info('Export completed')
