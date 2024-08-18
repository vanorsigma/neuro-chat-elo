import {
    InteractionResponseType,
    verifyKey,
} from 'discord-interactions';
import { addOptOut, removeOptOut } from "./firebase";
import { DiscordCommandInteraction } from './discordTypes';

export function handleUnknownDiscordType(): Response {
    console.error('Unknown Type for Discord Interaction');
    return new Response(JSON.stringify({ error: 'Unknown Type' }), {
        headers: {
            'content-type': 'application/json;charset=UTF-8',
        },
        status: 400 
    });
}

export function handlePing(): Response {
    return new Response(JSON.stringify({ type: InteractionResponseType.PONG }), {
        headers: {
            'content-type': 'application/json;charset=UTF-8',
        },
    });
}

export async function handleDiscordCommand(interaction: DiscordCommandInteraction, env: Env): Promise<Response> {
    const command = interaction.data.name;
    const userId = interaction.user.id;
    
    switch (command) {
        case 'opt_out':
            return await handleOptout(userId, env);
        case 'opt_in':
            return await handleOptin(userId, env);
        default:
            return new Response(JSON.stringify({ error: 'Unknown Type' }), {
                headers: {
                    'content-type': 'application/json;charset=UTF-8',
                },
                status: 400
            });
    }
}

async function handleOptout(userId: string, env: Env): Promise<Response> {
    await addOptOut(userId, 'discord', env);
    console.log(`Opting out ${userId} from discord`);
    return new Response(JSON.stringify({
        type: InteractionResponseType.CHANNEL_MESSAGE_WITH_SOURCE,
        data: {
            content: `You have been opted out of the leaderboards.`,
        },
    }), {
        headers: {
            'content-type': 'application/json;charset=UTF-8',
        },
    });
}

async function handleOptin(userId: string, env: Env): Promise<Response> {
    await removeOptOut(userId, 'discord', env);
    console.log(`Opting in ${userId} from discord`);
    return new Response(JSON.stringify({
        type: InteractionResponseType.CHANNEL_MESSAGE_WITH_SOURCE,
        data: {
            content: `You have been opted in to the leaderboards.`,
        },
    }), {
        headers: {
            'content-type': 'application/json;charset=UTF-8',
        },
    });
}

export async function verifyDiscord(discordPublicKey: string, request: Request, body: string): Promise<boolean | "" | null> {
    const signature = request.headers.get('X-Signature-Ed25519');
    const timestamp = request.headers.get('X-Signature-Timestamp');
    console.debug(`Verifying Discord request with signature: ${signature} and timestamp: ${timestamp}`);
    return signature && timestamp && (await verifyKey(body, signature, timestamp, discordPublicKey));
}