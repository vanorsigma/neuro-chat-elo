import {
    InteractionResponseType,
    InteractionType,
    verifyKey,
} from 'discord-interactions';
import { addOptOut, removeOptOut } from "./firebase";

export async function handleDiscordCommand(request: Request, env: Env) {
    const body = JSON.parse(await request.text());
    console.log(body)
    
    if (body.type === InteractionType.PING) {
        console.log('Recieved Ping interaction from Discord');
        return handlePing();
    }
    
    if (body.type === InteractionType.APPLICATION_COMMAND) {
        console.log('Recieved Application Command interaction from Discord');
        return await handleApplicationCommand(body, env);
    }
    
    console.error('Unknown Type');
    return new Response(JSON.stringify({ error: 'Unknown Type' }), {
        headers: {
            'content-type': 'application/json;charset=UTF-8',
        },
        status: 400 
    });
}

function handlePing(): Response {
    return new Response(JSON.stringify({ type: InteractionResponseType.PONG }), {
        headers: {
            'content-type': 'application/json;charset=UTF-8',
        },
    });
}

async function handleApplicationCommand(body: , env: Env): Promise<Response> {
    const command = body.data.name;
    const userId = body.member.user.id;
    
    switch (command) {
        case 'optout':
            return await handleOptout(userId, env);
        case 'optin':
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

export async function verifyDiscord(request: Request, env: Env) {
    const signature = request.headers.get('x-signature-ed25519');
    const timestamp = request.headers.get('x-signature-timestamp');
    return signature && timestamp && (await verifyKey(await request.text(), signature, timestamp, env.DISCORD_PUBLIC_KEY));
}