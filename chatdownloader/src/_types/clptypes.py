from dataclasses import dataclass
from typing import Any

from dataclasses_json import DataClassJsonMixin


@dataclass
class UserChatPerformance:
    id: str
    username: str
    avatar: str
    metrics: dict[str, int]
    metadata: dict[str, Any]


@dataclass
class BadgeInformation(DataClassJsonMixin):
    description: str
    image_url: str
