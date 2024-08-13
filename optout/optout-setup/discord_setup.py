from discord import Interaction, app_commands
import discord

import logging

logger = logging.getLogger(__name__)

__all__ = ("DiscordSetup",)

class DiscordSetup:
    client: discord.Client

    def __init__(self, client: discord.Client) -> None:
        self.client = client

    @classmethod
    async def from_token(cls, token: str):
        client = discord.Client(intents=discord.Intents.none())
        await client.login(token)
        return cls(client)
    
    # async def list_existing(self):
        # self.client.get_guild(647850202430046238).

    async def push_commands(self):
        command_tree = app_commands.CommandTree(self.client)


        @command_tree.command()
        async def opt_in(interaction: Interaction): ...

        @command_tree.command()
        async def opt_out(interaction: Interaction): ...

        @command_tree.command()
        async def about(interaction: Interaction): ...

        logger.debug(f"created command tree {command_tree}")
        synced = await command_tree.sync()
        # synced = await command_tree.sync(guild=discord.Object(id=647850202430046238))
        logger.info(f"synced command tree {synced}")
