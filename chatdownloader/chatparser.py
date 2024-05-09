"""
Parses chat logs retrieved by the TwitchDownloaderCLI executable

TODO: Need to read from history to figure out ranking, so we can calculate the new elo rating
"""

from typing import List, Dict, Any, Optional
from dataclasses import dataclass
from dataclasses_json import DataClassJsonMixin
import json
import os
import re

GIFTED_SUB_REGEX_1 = re.compile(r'(?P<gifter>[a-zA-Z0-9_]+) gifted a Tier (?P<tier>[0-9]) Sub to (?P<receiver>[a-zA-Z0-9_]+)!')
GIFTED_SUB_REGEX_2 = re.compile(r'(?P<gifter>[a-zA-Z0-9_]+) is gifting (?P<no_of_subs>[0-9]+) Tier (?P<tier>[0-9]) Subs to (?P<streamer>[a-zA-Z0-9_]+)\'s community!')
SCORE_WEIGHTS = {
    'bits': 0.02,  # reward more for bits, it's 100% to streamer
    'subs': 4.99,
    'text': 0.1,
    'emote': 0.05
}

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
    is_special_role: bool # VIPs / Mods
    message_length: int
    emotes_sent: bool


@dataclass
class EloInformation(DataClassJsonMixin):
    user_id: str
    display_name: str
    logo: str
    is_special_role: bool
    score: int


class ChatLogsParser:
    def __init__(self):
        pass

    def _no_of_gifted_subs(self, message: ChatMessageFragment) -> int:
        matches_1 = GIFTED_SUB_REGEX_1.match(message.text)
        matches_2 = GIFTED_SUB_REGEX_2.match(message.text)

        total = 0
        if matches_1 is not None:
            total += 1
        if matches_2 is not None:
            total += int(matches_2.group('no_of_subs'))
        return total

    def _parse_to_log_object(self, chat_log_path: str) -> ChatLog:
        with open(chat_log_path, 'r') as f:
            chat_logs = ChatLog.from_json(f.read())

        return chat_logs

    def _calculate_score(self, info: EloPreprocessedInformation) -> float:
        return info.bits_sent * SCORE_WEIGHTS['bits'] + \
            info.emotes_sent * SCORE_WEIGHTS['emote'] + \
            info.message_length * SCORE_WEIGHTS['text'] + \
            info.subs * SCORE_WEIGHTS['subs']

    def get_elo_information(
            self,
            infos: List[EloPreprocessedInformation]) -> Dict[str, EloInformation]:
        """
        Processes the preprocessed information to get the elo points of every user by ID

        :param infos: The preprocessed information
        :return: A dictionary tying a user ID to elo information
        """
        return_val = {}
        for info in infos:
            if info.user_id not in return_val:
                return_val[info.user_id] = EloInformation(info.user_id, info.display_name, info.logo, info.is_special_role, 0)

            elo_info = return_val[info.user_id]
            elo_info.score += self._calculate_score(info)

        return return_val

    def parse(self, chat_log_path: str) -> List[EloPreprocessedInformation]:
        """
        Parses the chat logs from the given path
        TODO: First and last messages should get extra elo points

        :param chat_log_path: The path to the chat log file
        :return: A list of EloPreprocessedInformation objects
        """
        chatlog = self._parse_to_log_object(chat_log_path)
        preprocessed_info = {}

        for comment in chatlog.comments:
            user_id = comment.commenter._id
            display_name = comment.commenter.display_name
            logo = comment.commenter.logo
            bits_sent = 0
            is_special_role = False
            message_length = 0
            emotes_sent = False

            for fragment in comment.message.fragments:
                bits_sent += fragment.bits_spent
                message_length += len(fragment.text)
                emotes_sent = len(fragment.emoticon) > 1

            preprocessed_info.append(EloPreprocessedInformation(user_id, display_name, logo, bits_sent, is_special_role, message_length, emotes_sent))

        return preprocessed_info


if __name__ == '__main__':
    clp = ChatLogsParser()
    chat_log = clp._parse('result.json')
    print(chat_log)
