"""
A main script that does backfilling given video IDs
"""

import logging
import asyncio
import os
from chatlogprocessor import ChatLogProcessor
from twitchdownloaderproxy import TwitchChatDownloader
from twitch_utils import get_auth_twitch

debug_mode = os.getenv('DEBUG')
logging.basicConfig(level=logging.DEBUG if bool(debug_mode) else logging.INFO)

VIDEO_IDS = [
    '2170316549',
    '2171991671',
    '2172878349',
    '2176205867',
    '2175349344'
]

twitch = asyncio.run(get_auth_twitch())
for video_id in VIDEO_IDS:
    # different tdp for temp management
    with TwitchChatDownloader() as tdp:
        logging.info('Backfilling %s', video_id)
        retrieved_chatlogs = tdp.download_chat(video_id)

        clp = ChatLogProcessor(twitch)
        performances = clp.parse_from_dict(retrieved_chatlogs)
        clp.export_to_leaderboards(performances)
