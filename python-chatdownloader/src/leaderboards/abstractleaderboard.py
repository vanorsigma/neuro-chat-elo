"""
Abstract Leaderboard
"""

import numpy as np
import logging
import json
import os

from abc import ABC
from typing import abstractmethod, Optional
from _types import (UserChatPerformance, LeaderboardInnerState,
                    LeaderboardExportItem)


K = 2.0


class AbstractLeaderboard(ABC):
    """
    A Leaderboard tells the program how to export a leaderboard
    """
    def __init__(self) -> None:
        self.state: list[LeaderboardInnerState] = {}
        self.read_initial_state()

    @classmethod
    @abstractmethod
    def get_name(cls) -> str:
        """
        Name of the leaderboard
        """

    @abstractmethod
    def calculate_score(self,
                        performance: UserChatPerformance) -> Optional[float]:
        """
        Calculates the score given the performance
        :param:performance:A normal performance
        :returns:A float if there is a score, None otherwise
        """

    def read_initial_state(self):
        """
        Reads the initial state of a leaderboard, and return a
        leaderboard object ready to digest information.
        """
        logging.info('Loading %s leaderboard...', self.get_name())
        if not os.path.exists(f'{self.get_name()}.json'):
            logging.info('%s leaderboard doesn\'t already exist.',
                         self.get_name())
            return

        with open(f'{self.get_name()}.json', 'r', encoding='utf8') as f:
            data = json.load(f)
            items = [
                LeaderboardExportItem.from_dict(item, infer_missing=True) for item in data]
            for item in items:
                self.state[item.id] = LeaderboardInnerState(
                    id=item.id,
                    username=item.username,
                    avatar=item.avatar,
                    elo=item.elo,
                    previous_rank=item.rank,
                    badges=item.badges,
                )
        logging.info('%s leaderboard loading ok', self.get_name())

    def update_leaderboard(self, performance: UserChatPerformance) -> None:
        """
        Updates the current leaderboard with the performance stated in
        the 'performance' variable

        :param: performance: An individual unit of performance
        """
        logging.debug('Updating %s leaderboard with performance: %s',
                      self.get_name(), performance)

        score = self.calculate_score(performance)
        logging.debug('Score for the above is %f', score)
        if score is None:
            return

        if performance.id not in self.state:
            self.state[performance.id] = LeaderboardInnerState(
                id=performance.id,
                username=performance.username,
                avatar=performance.avatar,
                badges=performance.metadata['badges'],
            )

        self.state[performance.id].score = score
        self.state[performance.id].badges = performance.metadata['badges']

    def __calculate_new_elo(self):
        # For every user, we make them fight 1000 users
        # (i.e. 1000 represented users)
        all_scores = [state.score for state in self.state.values()]
        mean = np.mean(all_scores)
        std = np.std(all_scores)

        sample_scores = np.random.normal(loc=mean, scale=std, size=500)
        sample_elos = [
            min(
                map(
                    lambda state: (state.elo, abs(state.score - score)),
                    self.state.values()), key=lambda t: t[1])[0]
            for score in sample_scores
        ]
        score_differences = {k.id: 0 for k in self.state.values()}

        logging.debug('Mean: %s, STD: %s', mean, std)
        logging.debug('Sample Scores: %s', sample_scores)
        logging.debug('Sample Elos: %s', sample_scores)

        for state in self.state.values():
            for idx, sample_score in enumerate(sample_scores):
                won = int(state.score > sample_score)
                p = (1.0 / (1.0 + 10 ** ((state.elo - sample_elos[idx]) / 400)))
                score_differences[state.id] += K * (won - p)

        # update all user's elo
        for uid, diff in score_differences.items():
            self.state[uid].elo += diff

    def save(self) -> None:
        """
        Saves the leaderboard.
        """
        logging.info('Saving %s leaderboard...', self.get_name())
        self.__calculate_new_elo()
        to_save = [LeaderboardExportItem(
            id=inner_state.id,
            rank=0,
            elo=inner_state.elo,
            username=inner_state.username,
            delta=0,
            avatar=inner_state.avatar,
            badges=inner_state.badges,
        ) for inner_state in self.state.values()]

        # update rank and delta
        to_save.sort(key=lambda x: x.elo, reverse=True)
        assert len(to_save) > 1, 'Nothing to save!'

        to_save[0].rank = 1
        to_save[0].delta = (
            (self.state[to_save[0].id].previous_rank - 1) if
            self.state[to_save[0].id].previous_rank is not None
            and
            self.state[to_save[0].id].previous_rank > 0 else 0)
        for idx, item in enumerate(to_save[1:]):
            rank = to_save[idx].rank + (int(item.elo < to_save[idx].elo))
            item.delta = ((self.state[item.id].previous_rank - rank) if
                          self.state[item.id].previous_rank is not None
                          and
                          self.state[item.id].previous_rank > 0 else 0)
            item.rank = rank

        with open(f'{self.get_name()}.json', 'w', encoding='utf8') as f:
            logging.info('Now writing to %s leaderboard...', self.get_name())
            json.dump([data.to_dict() for data in to_save], f)

        logging.info('Export completed')
