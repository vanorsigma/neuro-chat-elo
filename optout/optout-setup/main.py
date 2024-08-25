from __future__ import annotations
from typing import Tuple

import requests
import asyncio
import dotenv
import logging
import os
import twitch_setup
import json

dotenv.load_dotenv()
logging.basicConfig(level=logging.DEBUG)
log = logging.getLogger(__name__)

SECRETS = {
    "CLOUDFLARE_API_KEY": os.getenv("CLOUDFLARE_API_KEY"),
    "CLOUDFLARE_ACCOUNT_ID": os.getenv("CLOUDFLARE_ACCOUNT_ID"),
    "CLOUDFLARE_WORKER_NAME": os.getenv("CLOUDFLARE_WORKER_NAME"),
    "TWITCH_CLIENT_ID": os.getenv("TWITCH_CLIENT_ID"),
    "TWITCH_CLIENT_SECRET": os.getenv("TWITCH_CLIENT_SECRET"),
    "TWITCH_WEBHOOK_SECRET": os.getenv("TWITCH_WEBHOOK_SECRET"),
    "TWITCH_BOT_USERNAME": os.getenv("TWITCH_BOT_USERNAME"),
    "TWITCH_BOT_ID": os.getenv("TWITCH_BOT_ID"),
    "FIREBASE_CREDS": os.getenv("FIREBASE_CREDS"),
    "FIREBASE_PROJECT_ID": os.getenv("FIREBASE_PROJECT_ID"),
}


async def update_cloudflare_secret(
    secretName: str, secretValue: str, account_id: str, worker_name: str, api_key: str
) -> None:
    url = f"https://api.cloudflare.com/client/v4/accounts/{account_id}/workers/scripts/{worker_name}/secrets"
    body = json.dumps({"name": secretName, "text": secretValue, "type": "secret_text"})

    headers = {"Content-Type": "application/json", "Authorization": f"Bearer {api_key}"}

    response = requests.put(url, headers=headers, data=body)
    if not response.ok:
        raise Exception(
            f"Failed to update secret: {response.status_code} - {response.reason}"
        )
    log.info(f"Updated secret {secretName}")


async def setup_twitch(
    twitch_client_id: str, twitch_client_secret: str
) -> twitch_setup.TwitchSetup:
    log.info("reading twitch env variables")
    ts: twitch_setup.TwitchSetup = await twitch_setup.TwitchSetup.from_id_and_secret(
        twitch_client_id, twitch_client_secret
    )

    return ts


async def main():
    log.info("running twitch setup")
    ts: twitch_setup.TwitchSetup = await setup_twitch(
        SECRETS["TWITCH_CLIENT_ID"],
        SECRETS["TWITCH_CLIENT_SECRET"],
    )
    twitchUser, twitchRefresh = await ts.get_user_auth()
    SECRETS["TWITCH_USER_AUTH"] = twitchUser
    SECRETS["TWITCH_REFRESH_TOKEN"] = twitchRefresh

    log.info("running discord setup")
    # TODO: Next PR should add discord setup

    log.info("updating cloudflare secrets")
    for secret in SECRETS.keys():
        await update_cloudflare_secret(
            secret,
            SECRETS[secret],
            SECRETS["CLOUDFLARE_ACCOUNT_ID"],
            os.environ["CLOUDFLARE_WORKER_NAME"],
            SECRETS["CLOUDFLARE_API_KEY"],
        )
    # Must be run after worker has Twitch webhook secret
    await ts.create_whisper_webhook_sub(os.environ["CLOUDFLARE_WORKER_URL"])

    print("Setup complete")


if __name__ == "__main__":
    asyncio.run(main())
