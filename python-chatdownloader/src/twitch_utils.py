"""
Misc Twitch utilities
"""

import logging
import os

from twitchAPI.twitch import Twitch

TWITCH_APPID = os.getenv('TWITCH_APPID')
TWITCH_APPSECRET = os.getenv('TWITCH_APPSECRET')


async def get_auth_twitch() -> Twitch:
    return await Twitch(TWITCH_APPID, TWITCH_APPSECRET)


async def get_latest_vod(twitch: Twitch, ch_id: str) -> str:
    logging.info('Getting latest VOD')
    videos = []
    async for vid in twitch.get_videos(user_id=ch_id, first=1):
        videos.append(vid)
    assert len(videos) > 0
    logging.info('Will get chat for VOD %s', videos[0].id)
    return videos[0].id
