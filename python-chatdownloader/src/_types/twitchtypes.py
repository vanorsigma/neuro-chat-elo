"""
Contains all the Twitch types parsable from the chat log
"""

from dataclasses import dataclass
from typing import Any, Dict, List, Optional

from dataclasses_json import DataClassJsonMixin


@dataclass
class ChatMessageFragmentEmoticon(DataClassJsonMixin):
    """
    Represents an emoticon in a chat message fragment.
    """
    emoticon_id: str


@dataclass
class ChatMessageFragment(DataClassJsonMixin):
    """
    Represents a fragment of a chat message.
    """
    text: str
    emoticon: Optional[ChatMessageFragmentEmoticon]


@dataclass
class Badge(DataClassJsonMixin):
    """
    Represents a badge.
    """
    _id: str
    version: str


@dataclass
class ChatMessage(DataClassJsonMixin):
    """
    Represents a chat message.
    """
    body: str
    bits_spent: int
    fragments: List[ChatMessageFragment]
    user_badges: List[Badge]


@dataclass
class ChatUserInfo(DataClassJsonMixin):
    """
    Represents a user in a chat.
    """
    display_name: str
    _id: str
    logo: str


@dataclass
class Comment(DataClassJsonMixin):
    """
    Represents a comment in a chat.
    """
    _id: str
    message: ChatMessage
    commenter: ChatUserInfo


@dataclass
class ChatLog(DataClassJsonMixin):
    """
    Represents a chat log.
    """
    comments: List[Comment]
