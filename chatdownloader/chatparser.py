"""
Parses chat logs retrieved by the TwitchDownloaderCLI executable

TODO: Need to read from history to figure out ranking, so we can calculate the new elo rating
"""

import json
import logging
import os
import re
from dataclasses import dataclass
from typing import Any, Dict, List, Optional

from dataclasses_json import DataClassJsonMixin

# TODO: Override this basic config in the main script and remove this line from here.
# Ideally, I want to also log the function name
logging.basicConfig(level=logging.DEBUG)

GIFTED_SUB_REGEX_1 = re.compile(
    r"(?P<gifter>[a-zA-Z0-9_]+) gifted a Tier (?P<tier>[0-9]) Sub to (?P<receiver>[a-zA-Z0-9_]+)!"
)
GIFTED_SUB_REGEX_2 = re.compile(
    r"(?P<gifter>[a-zA-Z0-9_]+) is gifting (?P<no_of_subs>[0-9]+) Tier (?P<tier>[0-9]) Subs to (?P<streamer>[a-zA-Z0-9_]+)\'s community!"
)
SCORE_WEIGHTS = {
    "bits": 0.02,  # reward more for bits, it's 100% to streamer
    "subs": 4.99,
    "text": 0.1,
    "emote": 0.05,
}
SPECIAL_ROLES = ["vip", "moderator"]


@dataclass
class ChatMessageFragmentEmoticon(DataClassJsonMixin):
    emoticon_id: str


@dataclass
class ChatMessageFragment(DataClassJsonMixin):
    text: str
    emoticon: Optional[ChatMessageFragmentEmoticon]


@dataclass
class Badge(DataClassJsonMixin):
    _id: str
    version: str


@dataclass
class ChatMessage(DataClassJsonMixin):
    body: str
    bits_spent: int
    fragments: List[ChatMessageFragment]
    user_badges: List[Badge]


@dataclass
class ChatUserInfo(DataClassJsonMixin):
    display_name: str
    _id: str
    logo: str


@dataclass
class Comment(DataClassJsonMixin):
    _id: str
    message: ChatMessage
    commenter: ChatUserInfo


@dataclass
class ChatLog(DataClassJsonMixin):
    comments: List[Comment]


@dataclass
class EloPreprocessedInformation(DataClassJsonMixin):
    user_id: str
    display_name: str
    logo: str
    bits_sent: int
    is_special_role: bool  # VIPs / Mods
    message_length: int
    emotes_sent: bool
    subs: int


@dataclass
class Score(DataClassJsonMixin):
    user_id: str
    display_name: str
    logo: str
    is_special_role: bool
    score: float


class ChatLogsParser:
    @staticmethod
    def _no_of_gifted_subs(message: ChatMessageFragment) -> int:
        matches_1 = GIFTED_SUB_REGEX_1.match(message.text)
        matches_2 = GIFTED_SUB_REGEX_2.match(message.text)

        total = 0
        if matches_1 is not None:
            total += 1
        if matches_2 is not None:
            total += int(matches_2.group("no_of_subs"))
        return total

    def _parse_to_log_object(self, chat_log_path: str) -> ChatLog:
        with open(chat_log_path, "r", encoding="utf8") as f:
            chat_logs = ChatLog.from_json(f.read())

        return chat_logs

    def _calculate_score(self, info: EloPreprocessedInformation) -> float:
        return (
            info.bits_sent * SCORE_WEIGHTS["bits"]
            + info.emotes_sent * SCORE_WEIGHTS["emote"]
            + info.message_length * SCORE_WEIGHTS["text"]
            + info.subs * SCORE_WEIGHTS["subs"]
        )

    def get_consolidated_score(
        self, infos: List[EloPreprocessedInformation]
    ) -> Dict[str, Score]:
        """
        Processes the preprocessed information to get the elo points of every user by ID

        :param infos: The preprocessed information
        :return: A dictionary tying a user ID to elo information
        """
        logging.info("Getting consolidated score for all users...")
        return_val = {}
        for info in infos:
            if info.user_id not in return_val:
                return_val[info.user_id] = Score(
                    info.user_id, info.display_name, info.logo, info.is_special_role, 0
                )

            score_info = return_val[info.user_id]
            score_info.score += self._calculate_score(info)
            logging.debug("User %s's score is now %f", info.user_id, score_info.score)

        return return_val

    @staticmethod
    def __is_user_special_role(message: ChatMessage) -> bool:
        return any(badge._id in SPECIAL_ROLES for badge in message.user_badges)

    def parse_and_score(self, chat_log_path: str) -> Dict[str, Score]:
        """
        Parses the chat logs, and gets the consolidated score per user.

        :param chat_log_path: The path to the chat log file
        :return: A dictionary mapping user ID to the score received for the chat log
        """
        return self.get_consolidated_score(self.parse(chat_log_path))

    def parse(self, chat_log_path: str) -> List[EloPreprocessedInformation]:
        """
        Parses the chat logs from the given path
        TODO: First and last messages should get extra elo points

        :param chat_log_path: The path to the chat log file
        :return: A list of EloPreprocessedInformation objects
        """
        logging.info("Parsing chat logs...")
        chatlog = self._parse_to_log_object(chat_log_path)
        preprocessed_info = []

        for comment in chatlog.comments:
            logging.debug("Processing comment by %s", comment.commenter.display_name)
            user_id = comment.commenter._id
            display_name = comment.commenter.display_name
            logo = comment.commenter.logo
            bits_sent = comment.message.bits_spent
            is_special_role = self.__is_user_special_role(comment.message)
            message_length = 0
            emotes_sent = False
            subs = 0

            for fragment in comment.message.fragments:
                message_length += len(fragment.text)
                emotes_sent += int(fragment.emoticon is None)
                subs += self._no_of_gifted_subs(fragment)

            logging.debug(
                "Comment has: %s bits, %d length, %d emotes sent, %d subs gifted. User is %s",
                bits_sent,
                message_length,
                emotes_sent,
                subs,
                "special" if is_special_role else "not special",
            )

            preprocessed_info.append(
                EloPreprocessedInformation(
                    user_id,
                    display_name,
                    logo,
                    bits_sent,
                    is_special_role,
                    message_length,
                    emotes_sent,
                    subs,
                )
            )

        return preprocessed_info


if __name__ == "__main__":
    clp = ChatLogsParser()
    final_scores = clp.parse_and_score("result.json")
    print(
        list(sorted(final_scores.items(), key=lambda x: x[1].score, reverse=True))[:5]
    )
