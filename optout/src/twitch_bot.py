import os
import asyncio
from uuid import UUID
import firebase_admin
from firebase_admin import credentials, firestore
from dotenv import load_dotenv
from twitchAPI.pubsub import PubSub
from twitchAPI.twitch import Twitch
from twitchAPI.helper import first
from twitchAPI.type import AuthScope
from twitchAPI.oauth import UserAuthenticator

load_dotenv()

APP_ID = os.getenv("TWITCH_CLIENT_ID")
APP_SECRET = os.getenv("TWITCH_CLIENT_SECRET")
TARGET_CHANNEL = os.getenv("TWITCH_CHANNEL")

USER_SCOPE = [
    AuthScope.WHISPERS_READ,
    AuthScope.WHISPERS_EDIT,
    AuthScope.USER_MANAGE_WHISPERS,
]


class TwitchWhisperBot:
    """A bot for handling whispers on Twitch, including opt-in and opt-out functionalities."""

    def __init__(self, db):
        self.db = db
        self.twitch = None
        self.user = None

    async def callback_whisper(self, uuid: UUID, data: dict) -> None:
        """Handle received whispers based on commands."""
        if data["type"] != "whisper_received":
            return
        body = data["data_object"]["body"]
        from_id = data["data_object"]["from_id"]

        print(data)

        if body.startswith("!optout"):
            await self.handle_optout(from_id)

        elif body.startswith("!optin"):
            await self.handle_optin(from_id)

        else:
            # Default response
            await self.twitch.send_whisper(
                self.user.id,
                from_id,
                "Thanks for checking out the Neuro Chat Leaderboards! "
                "If you would like to opt out type !optout, and if you would like to opt back in type !optin.",
            )

    async def handle_optout(self, from_id):
        """Handle the opt-out process for a user."""
        if (
            self.db.collection("opt_outs")
            .where("id", "==", from_id)
            .where("platform", "==", "twitch")
            .get()
        ):
            await self.twitch.send_whisper(
                self.user.id,
                from_id,
                "You have already opted out of twitch leaderboards.",
            )
        else:
            self.db.collection("opt_outs").add({"id": from_id, "platform": "twitch"})
            await self.twitch.send_whisper(
                self.user.id,
                from_id,
                "You have successfully opted out of twitch leaderboards.",
            )

    async def handle_optin(self, from_id):
        """Handle the opt-in process for a user."""
        opt_outs = (
            self.db.collection("opt_outs")
            .where("id", "==", from_id)
            .where("platform", "==", "twitch")
            .get()
        )
        for opt_out in opt_outs:
            opt_out.reference.delete()
        await self.twitch.send_whisper(
            self.user.id,
            from_id,
            "You have successfully opted back in to twitch leaderboards.",
        )

    async def start(self):
        """Start the bot by initializing Twitch connection and listening for whispers."""
        self.twitch = Twitch(APP_ID, APP_SECRET)
        auth = UserAuthenticator(self.twitch, USER_SCOPE, force_verify=False)
        token, refresh_token = await auth.authenticate()
        await self.twitch.set_user_authentication(token, USER_SCOPE, refresh_token)
        self.user = await first(self.twitch.get_users())
        self.pubsub = PubSub(self.twitch)
        self.pubsub.start()
        await self.pubsub.listen_whispers(self.user.id, self.callback_whisper)

    async def stop(self):
        """Stop the bot by ending PubSub and closing Twitch connection."""
        self.pubsub.stop()
        await self.twitch.close()


if __name__ == "__main__":
    cred = credentials.Certificate("creds.json")
    firebase_admin.initialize_app(cred)
    bot = TwitchWhisperBot(firestore.client())
    asyncio.run(bot.start())
    input("Press ENTER to close...")
    asyncio.run(bot.stop())
