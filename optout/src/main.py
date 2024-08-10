import asyncio
import os
import firebase_admin
from firebase_admin import credentials, firestore

from discord_bot import DiscordBot
from twitch_bot import TwitchWhisperBot

if __name__ == "__main__":
    # Initialize the Firebase app
    cred = credentials.Certificate("creds.json")
    firebase_admin.initialize_app(cred)

    # Run the bots
    discord_bot = DiscordBot(firestore.client())
    twitch_bot = TwitchWhisperBot(firestore.client())

    loop = asyncio.get_event_loop()
    loop.run_until_complete(
        asyncio.gather(
            discord_bot.start(os.getenv("DISCORD_TOKEN")), twitch_bot.start()
        )
    )
    loop.close()
