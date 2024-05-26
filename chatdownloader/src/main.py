"""
Script entrypoint
"""

import logging

from .twitch_utils import get_auth_twitch, get_latest_vod
from .chatlogprocessor import ChatLogProcessor
from .twitchdownloaderproxy import TwitchChatDownloader

logging.basicConfig(level=logging.INFO)

VED_CH_ID = '85498365'


if __name__ !='__main__':
    print('This script is meant to be run independently')

logging.info('Script triggered, pulling latest Twitch VOD')
video_id = get_latest_vod(get_auth_twitch(), VED_CH_ID)
logging.info('Pulling chat logs')

with TwitchChatDownloader() as tdp:
    retrieved_chatlogs = tdp.download_chat(video_id)

clp = ChatLogProcessor()
performances = clp.parse_from_dict(retrieved_chatlogs)
clp.export_to_leaderboards(performances)
