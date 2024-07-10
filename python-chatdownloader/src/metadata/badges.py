"""
Assigns badges to each user
"""


import logging
import asyncio
from typing import Any

from _types import BadgeInformation, Comment
from consts import VED_CH_ID
from twitchAPI.twitch import Twitch

from .abstractmetadata import AbstractMetadata


class Badges(AbstractMetadata):
    """
    Gets the user badges from the chat log, figures out the respective
    image url for the badges, and returns them
    """

    def __init__(self, twitch: Twitch) -> None:
        super().__init__(twitch)
        self.badges = self.__get_all_applicable_badges()
        logging.debug("Badges: %s", self.badges)

    def __get_all_applicable_badges(self) -> dict[str, dict[str,
                                                            BadgeInformation]]:
        logging.info("Getting all applicable badges")
        return {
            x.set_id: {
                y.id: BadgeInformation(y.description, y.image_url_4x)
                for y in x.versions
            }
            for x in [*asyncio.run(self.twitch.get_global_chat_badges()),
                      *asyncio.run(self.twitch.get_chat_badges(VED_CH_ID))]}

    @classmethod
    def get_name(cls) -> str:
        return 'badges'

    @classmethod
    def get_default_value(cls) -> Any:
        return {}

    def get_metadata(
            self, comment: Comment, sequence_no: int) -> dict[str, Any]:
        user = comment.commenter._id
        badges = comment.message.user_badges
        return {user: [self.badges[badge._id][badge.version] for badge in badges
                       if badge._id in self.badges and
                       badge.version in self.badges[badge._id]]}
