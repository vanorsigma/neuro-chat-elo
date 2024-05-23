"""
The overall leaderboard
"""

import os
import json
import logging
from dataclasses import dataclass

from .abstractleaderboard import AbstractLeaderboard
from _types import LeaderboardExportItem, UserChatPerformance

DEFAULT_ELO = 1200
K = 0.5

@dataclass
class OverallLeaderboardInnerState:
    """
    Special Inner State to calculate elo
    """
    id: str
    username: str
    avatar: str
    previous_rank: int = 999999  # surely we won't get more than a million viewers
    elo: float = DEFAULT_ELO
    score: int = 0


class Overall(AbstractLeaderboard):
    """
    Computes the overall leaderboard (Special + Non-Special).
    """
    def __init__(self):
        self.state = {}
        super().__init__()

    def read_initial_state(self):
        logging.info('Loading overall leaderboard...')
        if not os.path.exists('leaderboard.json'):
            logging.info('Overall leaderboard doesn\'t already exist.')
            return

        with open('leaderboard.json', 'r', encoding='utf8') as f:
            data = json.load(f)
            items = [
                LeaderboardExportItem.from_dict(item) for item in data]
            for item in items:
                self.state[item.id] = OverallLeaderboardInnerState(
                    id=item.id,
                    username=item.username,
                    avatar=item.avatar,
                    elo=item.elo
                )
        logging.info('Overall leaderboard loading ok')

    def update_leaderboard(self, performance: UserChatPerformance) -> None:
        if performance.id not in self.state:
            self.state[performance.id] = OverallLeaderboardInnerState(
                id=performance.id,
                username=performance.username,
                avatar=performance.avatar,
            )

        self.state[performance.id].score = sum(performance.metrics.values())

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
                p_1 = (1.0 / (1.0 + 10**((inner_state_1.elo - inner_state_2.elo) / 400)))
                p_2 = (1.0 / (1.0 + 10**((inner_state_2.elo - inner_state_1.elo) / 400)))

                # slight variant: if tie, we award elo to both
                p1_won = int(inner_state_1.score >= inner_state_2.score)
                p2_won = int(inner_state_2.score >= inner_state_1.score)
                inner_state_1.elo += K * (p1_won - p_1)
                inner_state_2.elo += K * (p2_won - p_2)

                versus_complete.add((inner_state_1.id, inner_state_2.id))

    def save(self):
        logging.info('Now saving overall leaderboard...')

        # figure out everyone's new elo
        self.__calculate_new_elo()

        # convert inner state to save state
        to_save = [LeaderboardExportItem(
            id=inner_state.id,
            rank=0,
            elo=inner_state.elo,
            username=inner_state.username,
            delta=0,
            avatar=inner_state.avatar
        ) for inner_state in self.state.values()]

        # update rank and delta
        to_save.sort(key=lambda x: x.elo, reverse=True)
        assert len(to_save) > 1, 'Nothing to save!'

        to_save[0].rank = 1
        for idx, item in enumerate(to_save[1:]):
            rank = to_save[idx].rank + (int(item.elo < to_save[idx].elo))
            item.delta = item.previous_rank - rank
            item.rank = rank

        with open('leaderboard.json', 'w', encoding='utf8') as f:
            logging.info('Now writing to overall leaderboard...')
            json.dump([data.to_dict() for data in to_save], f)

        logging.info('Export completed')
