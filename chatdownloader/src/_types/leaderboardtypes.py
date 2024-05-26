"""
Publically accessible leaderboard types
"""
from typing import Optional
from dataclasses import dataclass
from dataclasses_json import DataClassJsonMixin

DEFAULT_ELO = 1200

@dataclass
class LeaderboardExportItem(DataClassJsonMixin):
    id: str
    rank: int
    elo: float
    username: str
    delta: float
    avatar: str


@dataclass
class LeaderboardInnerState:
    id: str
    username: str
    avatar: str
    previous_rank: Optional[int] = None
    elo: float = DEFAULT_ELO
    score: int = 0
