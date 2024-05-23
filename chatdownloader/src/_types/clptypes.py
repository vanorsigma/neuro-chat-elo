from dataclasses import dataclass
from typing import Any

@dataclass
class UserChatPerformance:
    id: str
    username: str
    avatar: str
    metrics: dict[str, int]
    metadata: dict[str, Any]
