from __future__ import annotations

import asyncio
import logging
import os

import dotenv

import discord_setup
import twitch_setup

dotenv.load_dotenv()
logging.basicConfig(level=logging.DEBUG)
log = logging.getLogger(__name__)


async def setup_twitch():
    log.info("reading twitch env variables")
    twitch_app_id = os.environ["TWITCH_CLIENT_ID"]
    twitch_app_secret = os.environ["TWITCH_CLIENT_SECRET"]
    ts: twitch_setup.TwitchSetup = await twitch_setup.TwitchSetup.from_id_and_secret(
        twitch_app_id, twitch_app_secret
    )
    subs = await ts.get_whisper_event_subs()

    log.info([sub.transport['callback'] for sub in subs])

    if not any(sub.transport['callback'] == f"{os.environ["WORKER_URL"]}/twitch" for sub in subs):
        log.info("creating whisper webhook subscription")
        await ts.create_whisper_webhook_sub(f"{os.environ["WORKER_URL"]}/twitch")
    else:
        log.info("whisper webhook subscription already exists")

    log.info([f"${sub.transport}" for sub in subs])


async def setup_discord():
    log.info("reading discord env variables")
    discord_token = os.environ["DISCORD_TOKEN"]
    ds = await discord_setup.DiscordSetup.from_token(discord_token)
    await ds.push_commands()
    await ds.client.close()


async def main():
    log.info("running twitch setup")
    await setup_twitch()
    # log.info("running discord setup")
    # await setup_discord()


asyncio.run(main())
