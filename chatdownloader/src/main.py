"""
Script entrypoint
"""

import logging

import asyncio

from twitch_utils import get_auth_twitch, get_latest_vod
from chatlogprocessor import ChatLogProcessor
from twitchdownloaderproxy import TwitchChatDownloader

logging.basicConfig(level=logging.INFO)

VED_CH_ID = '85498365'


if __name__ != '__main__':
    print('This script is meant to be run independently')

async def get_video_id():
    twitch = await get_auth_twitch()
    return await get_latest_vod(twitch, VED_CH_ID)

logging.info('Script triggered, pulling latest Twitch VOD')
video_id = asyncio.run(get_video_id())
logging.info('Pulling chat logs for %s', video_id)

with TwitchChatDownloader() as tdp:
    retrieved_chatlogs = tdp.download_chat(video_id)

clp = ChatLogProcessor()
performances = clp.parse_from_dict(retrieved_chatlogs)
clp.export_to_leaderboards(performances)
