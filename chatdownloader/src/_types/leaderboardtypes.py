"""
Publically accessible leaderboard types
"""
from dataclasses import dataclass
from dataclasses_json import DataClassJsonMixin

@dataclass
class LeaderboardExportItem(DataClassJsonMixin):
    id: str
    rank: int
    elo: float
    username: str
    delta: float
    avatar: str
