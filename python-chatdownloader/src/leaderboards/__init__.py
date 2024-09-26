"""
Leaderboards
"""

from .overall import Overall
from .chatonly import ChatOnly
from .copypastaleaders import CopypastaLeaders
from .nonvips import NonVIPS
from .bitsonly import BitsOnly
from .subsonly import SubsOnly


EXPORTED_LEADERBOARDS = [Overall, ChatOnly, CopypastaLeaders,
                         NonVIPS, BitsOnly, SubsOnly]
