"""
Script entrypoint
"""

import asyncio
import logging
import os

from chatlogprocessor import ChatLogProcessor
from consts import VED_CH_ID
from twitch_utils import get_auth_twitch, get_latest_vod
from twitchdownloaderproxy import TwitchChatDownloader

debug_mode = os.getenv('DEBUG')
logging.basicConfig(level=logging.DEBUG if bool(debug_mode) else logging.INFO)


if __name__ != '__main__':
    print('This script is meant to be run independently')

logging.info('Authenticating twitch...')
twitch = asyncio.run(get_auth_twitch())

async def get_video_id():
    return await get_latest_vod(twitch, VED_CH_ID)

logging.info('Script triggered, pulling latest Twitch VOD')
video_id = asyncio.run(get_video_id())
logging.info('Pulling chat logs for %s', video_id)

with TwitchChatDownloader() as tdp:
    retrieved_chatlogs = tdp.download_chat(video_id)

clp = ChatLogProcessor(twitch)
performances = clp.parse_from_dict(retrieved_chatlogs)
clp.export_to_leaderboards(performances)
