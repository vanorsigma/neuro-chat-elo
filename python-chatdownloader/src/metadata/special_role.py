"""
Figures out if the user is a special role
"""

from typing import Any

from _types import Comment

from .abstractmetadata import AbstractMetadata

SPECIAL_ROLES = ['vip', 'moderator']

class SpecialRole(AbstractMetadata):
    """
    Figures out if the the user is a special role
    """
    @classmethod
    def get_name(cls) -> str:
        return 'special_role'

    @classmethod
    def get_default_value(cls) -> Any:
        return False

    def get_metadata(
            self, comment: Comment, sequence_no: int) -> dict[str, Any]:
        is_special_role = any(badge._id in SPECIAL_ROLES for badge in
                    comment.message.user_badges)
        if is_special_role:
            return {comment.commenter._id: True}
        return {}
