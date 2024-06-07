"""
A main script that does backfilling given video IDs
"""

import logging
from chatlogprocessor import ChatLogProcessor
from twitchdownloaderproxy import TwitchChatDownloader

logging.basicConfig(level=logging.DEBUG)

VIDEO_IDS = [
    '2163534622',
]

with TwitchChatDownloader() as tdp:
    for video_id in VIDEO_IDS:
        logging.info('Backfilling %s', video_id)
        retrieved_chatlogs = tdp.download_chat(video_id)

        clp = ChatLogProcessor()
        performances = clp.parse_from_dict(retrieved_chatlogs)
        clp.export_to_leaderboards(performances)
