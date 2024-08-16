from __future__ import annotations
from typing import Tuple

import aiohttp
import asyncio
import dotenv
import logging
import os
import discord_setup
import twitch_setup
import json

dotenv.load_dotenv()
logging.basicConfig(level=logging.DEBUG)
log = logging.getLogger(__name__)

SECRETS = [
    "CLOUDFLARE_API_KEY",
    "CLOUDFLARE_ACCOUNT_ID",
    "TWITCH_CLIENT_ID",
    "TWITCH_CLIENT_SECRET",
    "TWITCH_WEBHOOK_SECRET",
    "DISCORD_BOT_TOKEN",
    "FIREBASE_CREDS",
]

async def updateCloudflareSecret(secretName: str, secretValue: str) -> None:
    url = f"https://api.cloudflare.com/client/v4/accounts/{os.environ["CLOUDFLARE_ACCOUNT_ID"]}/workers/scripts/{os.environ["CLOUDFLARE_WORKER_NAME"]}/secrets"
    body = json.dumps({
        "name": secretName,
        "text": secretValue,
        "type": "secret_text"
    })

    headers = {
        "Content-Type": "application/json",
        "Authorization": f"Bearer {os.environ["CLOUDFLARE_API_KEY"]}"
    }

    async with aiohttp.ClientSession() as session:
        async with session.put(url, headers=headers, data=body) as response:
            if not response.ok:
                raise Exception(f"Failed to update secret: {response.status} - {response.reason}")
            log.info(f"Updated secret {secretName}")


async def setup_twitch() -> Tuple[str, str]:
    log.info("reading twitch env variables")
    twitch_app_id = os.environ["TWITCH_CLIENT_ID"]
    twitch_app_secret = os.environ["TWITCH_CLIENT_SECRET"]
    ts: twitch_setup.TwitchSetup = await twitch_setup.TwitchSetup.from_id_and_secret(
        twitch_app_id, twitch_app_secret
    )
    subs = await ts.get_whisper_event_subs()

    log.info([sub.transport['callback'] for sub in subs])

    if not any(sub.transport['callback'] == f"{os.environ["CLOUDFLARE_WORKER_URL"]}/twitch" for sub in subs):
        log.info("creating whisper webhook subscription")
        await ts.create_whisper_webhook_sub(f"{os.environ["CLOUDFLARE_WORKER_URL"]}/twitch")
    else:
        log.info("whisper webhook subscription already exists")

    log.info([f"${sub.transport}" for sub in subs])

    return await ts.get_user_auth()


async def setup_discord():
    log.info("reading discord env variables")
    discord_token = os.environ["DISCORD_BOT_TOKEN"]
    ds = await discord_setup.DiscordSetup.from_token(discord_token)
    await ds.push_commands()
    await ds.client.close()


async def main():
    log.info("running twitch setup")
    twitchUser, twitchRefresh = await setup_twitch()
    log.info("running discord setup")
    await setup_discord()

    log.info("updating cloudflare secrets")
    for secret in SECRETS:
        await updateCloudflareSecret(secret, os.environ[secret])
    await updateCloudflareSecret("TWITCH_USER_AUTH", twitchUser)
    await updateCloudflareSecret("TWITCH_REFRESH_TOKEN", twitchRefresh)
    print("Setup complete")




asyncio.run(main())
