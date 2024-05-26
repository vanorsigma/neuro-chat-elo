"""
Leaderboards
"""

from .overall import Overall
from .chatonly import ChatOnly
from .copypastaleaders import CopypastaLeaders
from .nonvips import NonVIPS

EXPORTED_LEADERBOARDS = [Overall, ChatOnly, CopypastaLeaders, NonVIPS]
