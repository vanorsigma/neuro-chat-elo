import os
import asyncio
import discord
import firebase_admin
from firebase_admin import credentials, firestore
from dotenv import load_dotenv

load_dotenv()


class DiscordBot(discord.Client):
    """A Discord bot that handles opt-in and opt-out commands to manage user preferences."""

    def __init__(self, db):
        """Initialize the bot with database access and set up intents."""
        intents = discord.Intents.default()
        intents.message_content = True
        super().__init__(intents=intents)
        self.db = db

    async def on_ready(self):
        """Prints a message when the bot is fully logged in and ready."""
        print(f"Logged in as {self.user}")

    async def on_message(self, message):
        """Responds to messages with commands for opting in and out."""
        if isinstance(message.channel, discord.DMChannel):
            if message.author == self.user:
                return

            command = message.content.split()[0]
            user_id = str(message.author.id)
            response = ""

            if command == "!optout":
                response = await self.handle_optout(user_id)

            elif command == "!optin":
                response = await self.handle_optin(user_id)

            else:
                response = (
                    "Thanks for checking out the Neuro Chat Leaderboards! "
                    "If you would like to opt out type !optout, and if you would like to opt back in type !optin."
                )

            await message.channel.send(response)

    async def handle_optout(self, user_id):
        """Handles user opt-out requests."""
        if (
            self.db.collection("opt_outs")
            .where("id", "==", user_id)
            .where("platform", "==", "discord")
            .get()
        ):
            return "You have already opted out of twitch leaderboards."
        self.db.collection("opt_outs").add({"id": user_id, "platform": "discord"})
        return "You have successfully opted out of twitch leaderboards."

    async def handle_optin(self, user_id):
        """Handles user opt-in requests."""
        opt_outs = (
            self.db.collection("opt_outs")
            .where("id", "==", user_id)
            .where("platform", "==", "discord")
            .get()
        )
        for opt_out in opt_outs:
            opt_out.reference.delete()
        return f"You have successfully opted back in to discord leaderboards."


if __name__ == "__main__":
    cred = credentials.Certificate("creds.json")
    firebase_admin.initialize_app(cred)
    bot = DiscordBot(firestore.client())
    bot.run(os.getenv("DISCORD_TOKEN"))
