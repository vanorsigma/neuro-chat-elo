"""
Represents an abstract metadata

(NOTE: Designed this way so that we can do multiprocessing eventually)
"""

from abc import ABC, abstractmethod
from typing import Any

from _types import Comment


class AbstractMetadata(ABC):
    """
    Represents an abstract metadata class.
    """
    @classmethod
    @abstractmethod
    def get_name(cls) -> str:
        """
        Name of this piece of metadata
        """

    @classmethod
    @abstractmethod
    def get_default_value(cls) -> Any:
        """
        A default value
        """

    @abstractmethod
    def get_metadata(
            self, comment: Comment, sequence_no: int) -> dict[str, Any]:
        """
        Get information about a user from a chat message

        :param: comment A comment from the user
        :returns: A partial update to a user's metadata (dictionary of
                  username to value)
        """
