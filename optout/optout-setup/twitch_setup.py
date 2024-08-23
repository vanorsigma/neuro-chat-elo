import asyncio
import os
from typing import List, Self, Tuple
import logging
import requests
from twitchAPI.twitch import Twitch, EventSubSubscription
from twitchAPI.type import AuthScope
from twitchAPI.oauth import UserAuthenticator

__all__ = ("TwitchSetup",)

log = logging.getLogger(__name__)

TWITCH_AUTH_SCOPES = [AuthScope.USER_MANAGE_WHISPERS]


class TwitchSetup:
    api: Twitch

    def __init__(self, twitch_api: Twitch) -> None:
        self.api: Twitch = twitch_api

    @classmethod
    async def from_id_and_secret(cls, client_id: str, client_secret: str) -> Self:
        twitch = Twitch(client_id, client_secret)
        await twitch.authenticate_app(
            [
                AuthScope.USER_MANAGE_WHISPERS,
                AuthScope.WHISPERS_READ,
                AuthScope.WHISPERS_EDIT,
            ]
        )

        log.debug(twitch._app_auth_token)
        log.debug(twitch._user_auth_token)
        return cls(twitch)

    async def get_user_auth(self) -> Tuple[str, str]:
        auth = UserAuthenticator(
            self.api,
            TWITCH_AUTH_SCOPES,
            force_verify=False,
        )
        # Start the authentication server and wait until user authenticates
        auth._start()
        while not auth._server_running:
            await asyncio.sleep(0.01)
        print(auth.return_auth_url())
        while auth._user_token is None:
            await asyncio.sleep(0.01)

        token, refresh_token = await auth.authenticate(user_token=auth._user_token)
        log.debug(f"TWITCH_USER_AUTH: {token}")
        log.debug(f"TWITCH_REFRESH_TOKEN: {refresh_token}")
        await self.api.set_user_authentication(
            token,
            TWITCH_AUTH_SCOPES,
            refresh_token,
        )
        return token, refresh_token

    async def remove_whisper_event_subs(self) -> None:
        subs = await self.get_whisper_event_subs()
        for sub in subs:
            await self.api.delete_eventsub_subscription(sub.id)

    async def get_whisper_event_subs(self) -> List[EventSubSubscription]:
        result = await self.api.get_eventsub_subscriptions(
            sub_type="user.whisper.message"
        )
        return result.data

    async def create_whisper_webhook_sub(self, url: str) -> None:
        users = await anext(self.api.get_users())
        userId = users.id
        secret = os.getenv("TWITCH_WEBHOOK_SECRET")

        log.debug(f"Create whisper webhook subscription for {userId} at {url}")

        headers = {
            "Client-ID": self.api.app_id,
            "Authorization": f"Bearer {self.api._app_auth_token}",
            "Content-Type": "application/json",
        }

        payload = {
            "type": "user.whisper.message",
            "version": "1",
            "condition": {"user_id": userId},
            "transport": {"method": "webhook", "callback": url, "secret": secret},
        }

        response = requests.post(
            "https://api.twitch.tv/helix/eventsub/subscriptions",
            headers=headers,
            json=payload,
        )

        if not response.ok:
            raise Exception(response.text)

        return response.ok
