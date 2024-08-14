export interface DiscordBaseInteraction {
    app_permissions: string;
    application_id: string;
    authorizing_integration_owners: Record<string, string>;
    entitlements: any[];
    id: string;
    token: string;
    type: number;
    version: number;
}

interface DiscordUser {
    avatar: string;
    avatar_decoration_data: null;
    clan: null;
    discriminator: string;
    global_name: string;
    id: string;
    public_flags: number;
    username: string;
}

interface DiscordChannel {
    flags: number;
    id: string;
    last_message_id: string;
    recipients: DiscordUser[];
    type: number;
}

interface DiscordInteractionData {
    id: string;
    name: string;
    type: number;
}

// For the Ping Interaction
export interface DiscordPingInteraction extends DiscordBaseInteraction {
    user: DiscordUser;
}

// For Command Interactions
export interface DiscordCommandInteraction extends DiscordBaseInteraction {
    context: number;
    data: DiscordInteractionData;
    locale: string;
    user: DiscordUser;
}

// For Command Interactions with channel data
export interface DiscordChannelCommandInteraction extends DiscordCommandInteraction {
    channel: DiscordChannel;
    channel_id: string;
}

export type DiscordInteraction = DiscordPingInteraction | DiscordCommandInteraction | DiscordChannelCommandInteraction;
