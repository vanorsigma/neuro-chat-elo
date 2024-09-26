"""
The emote metrics
"""
import logging
from dataclasses import dataclass

import requests
from _types import ChatMessageFragment, Comment
from consts import VED_CH_ID

from .abstractmetric import AbstractMetric

WEIGHT_EMOTES = 0.02
SEVEN_TV_URL = f"https://7tv.io/v3/users/twitch/{VED_CH_ID}"


@dataclass
class SevenTVEmote:
    name: str
    # NOTE: Will need this in the future, but it's only here now
    # because I'm lazy to rediscover where to get the image url
    emote_url: str


class Emote(AbstractMetric):
    """
    The emote metric.
    """

    def __init__(self):
        self.seventv_emotes = self.__get_all_7tv_emotes()
        self.seventv_lookup = {emote.name for emote in self.seventv_emotes}

    @staticmethod
    def __get_all_7tv_emotes() -> list[SevenTVEmote]:
        logging.info('Getting the 7TV channel emotes')
        response = requests.get(SEVEN_TV_URL, timeout=5)
        if not response.ok:
            logging.warning('Cannot get 7tv emotes')
            return []

        resp_body = response.json()
        ret_val = []
        try:
            raw_emotes = resp_body['emote_set']['emotes']
            for raw_emote in raw_emotes:
                host_url = raw_emote['data']['host']['url']
                filename = max(filter(lambda x: x['name'].endswith('.webp'),
                                      raw_emote['data']['host']['files']),
                               key=lambda emote: emote['width'])
                ret_val.append(SevenTVEmote(
                    name=raw_emote['name'],
                    emote_url=f'https://{host_url}/{filename}'
                ))
        except KeyError as e:
            logging.error('Cannot access the required keys to get the emotes',
                          exc_info=e)
            return []

        logging.info('Got %d 7tv emotes', len(ret_val))
        return ret_val

    def __count_7tv_emotes_in_fragment(self, fragment: ChatMessageFragment):
        count = 0
        for word in fragment.text.split(' '):
            if word in self.seventv_lookup:
                count += 1
        logging.debug('Found %d number of 7TV emotes in %s',
                      count, fragment.text)
        return count

    @classmethod
    def can_parallelize(cls) -> bool:
        return False

    @classmethod
    def get_name(cls) -> str:
        return 'emote'

    def get_metric(self, comment: Comment,
                   sequence_no: int) -> dict[str, float]:
        return self._shortcut_for_this_comment_user(
            comment,
            sum(int(fragment.emoticon is not None) +
                self.__count_7tv_emotes_in_fragment(fragment)
                for fragment in comment.message.fragments) * WEIGHT_EMOTES)
