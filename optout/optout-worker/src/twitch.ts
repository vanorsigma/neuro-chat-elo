// FIXME: Twitch User Auth tokens expire, need to refresh with refresh token and update stored auth token sadge

import { addOptOut, removeOptOut } from "./firebase";

const TWITCH_MESSAGE_ID = 'Twitch-Eventsub-Message-Id'.toLowerCase();
const TWITCH_MESSAGE_TIMESTAMP = 'Twitch-Eventsub-Message-Timestamp'.toLowerCase();
const TWITCH_MESSAGE_SIGNATURE = 'Twitch-Eventsub-Message-Signature'.toLowerCase();

const HMAC_PREFIX = 'sha256=';

interface Whisper {
    text: string;
}

interface WhisperEvent {
    from_user_id: string;
    from_user_name: string;
    whisper: Whisper;
}

export interface TwitchNotification {
    event: WhisperEvent;
}

async function getHmac(secret: string, message: string): Promise<string> {
    const encoder = new TextEncoder();
    const keyData = encoder.encode(secret);
    const messageData = encoder.encode(message);

    const key = await crypto.subtle.importKey(
        'raw',
        keyData,
        { name: 'HMAC', hash: 'SHA-256' },
        false,
        ['sign']
    );

    const signature = await crypto.subtle.sign('HMAC', key, messageData);
    const hashArray = Array.from(new Uint8Array(signature));
    const hashHex = hashArray.map(byte => byte.toString(16).padStart(2, '0')).join('');
    return hashHex;
}

function verifyMessage(hmac: string, verifySignature: string): boolean {
    const hmacBuffer = new Uint8Array(hmac?.match(/.{1,2}/g)?.map(byte => parseInt(byte, 16)) || []);
    const verifySignatureBuffer = new Uint8Array(verifySignature?.match(/.{1,2}/g)?.map(byte => parseInt(byte, 16)) || []);

    if (hmacBuffer.length !== verifySignatureBuffer.length) {
        return false;
    }

    let diff = 0;
    for (let i = 0; i < hmacBuffer.length; i++) {
        diff |= hmacBuffer[i] ^ verifySignatureBuffer[i];
    }

    return diff === 0;
}

function getHmacMessage(headers: Headers, body: string): string | null {
    const twitchMessageId = headers.get(TWITCH_MESSAGE_ID);
    const twitchMessageTimestamp = headers.get(TWITCH_MESSAGE_TIMESTAMP);

    if (twitchMessageId == null || twitchMessageTimestamp == null) {
        return null;
    }

    return twitchMessageId + twitchMessageTimestamp + body
}

export async function verifyTwitch(twitchWebhookSecret: string, headers: Headers, requestBody: string): Promise<boolean> {
    const message = getHmacMessage(headers, requestBody);

    if (message == null) {
        return false;
    }

    const hmac = HMAC_PREFIX + getHmac(twitchWebhookSecret, message);
    const messageSignature = headers.get(TWITCH_MESSAGE_SIGNATURE);

    if (messageSignature == null) {
        return false;
    }

    return verifyMessage(hmac, messageSignature);
}

export async function handleWhisper(event: WhisperEvent, env: Env): Promise<CommandResponse> {
    const text = event.whisper.text;
    const user = event.from_user_name;
    const userId = event.from_user_id;

    console.log(`Received whisper from ${user} (${userId}): ${text}`);

    switch (text) {
        case '/optout':
            return await handleOptout(userId, env);
        case '/optin':
            return await handleOptin(userId, env);
        default:
            return {
                success: true,
            }
    }
}

async function handleOptout(userId: string, env: Env): Promise<CommandResponse> {
    console.log(`Opting out ${userId}`);
    const result = await addOptOut(userId, 'twitch', env);
    await sendWhisper(userId, 'You have been opted out of the leaderboards', env);
    return result;
}

async function handleOptin(userId: string, env: Env): Promise<CommandResponse> {
    console.log(`Opting in ${userId}`);
    const result = await removeOptOut(userId, 'twitch', env);
    await sendWhisper(userId, 'You have been opted back into the leaderboards', env);
    return result;
}

async function sendWhisper(userId: string, text: string, env: Env): Promise<void> {
    const url = `https://api.twitch.tv/helix/whispers?from_user_id=${env.TWITCH_BOT_ID}&to_user_id=${userId}&message=${encodeURIComponent(text)}`;
    const response = await fetch(url, {
        method: 'POST',
        headers: {
            'Client-ID': env.TWITCH_CLIENT_ID,
            'Authorization': `Bearer ${env.TWITCH_USER_AUTH}`
        }
    });

    if (response.status == 401) {
        const { accessToken, refreshToken } = await refreshTwitchToken(env.TWITCH_CLIENT_ID, env.TWITCH_CLIENT_SECRET, env.TWITCH_REFRESH_TOKEN);
        await updateSecret('TWITCH_USER_AUTH', accessToken, env);
        await updateSecret('TWITCH_REFRESH_TOKEN', refreshToken, env);
        const response = await fetch(url, {
            method: 'POST',
            headers: {
                'Client-ID': env.TWITCH_CLIENT_ID,
                'Authorization': `Bearer ${env.TWITCH_USER_AUTH}`
            }
        });
    }

    if (!response.ok) {
        throw new Error(`Failed to send whisper to ${userId}: ${response.status} - ${response.statusText}`);
    }
}

async function refreshTwitchToken(twitchClientId: string, twitchClientSecret: string, refreshToken: string): Promise<{ accessToken: string, refreshToken: string }> {
    const url = "https://id.twitch.tv/oauth2/token";
    const body = new URLSearchParams({
        grant_type: "refresh_token",
        refresh_token: refreshToken,
        client_id: twitchClientId,
        client_secret: twitchClientSecret,
    })
    
    const response = await fetch(url, {
        method: "POST",
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded'
        },
        body: body.toString()
    });

    const data = await response.json();
    if (response.ok) {
        return { accessToken: data.access_token, refreshToken: data.refresh_token };
    } else {
        throw new Error(`Failed to refresh token: ${response.status} - ${response.statusText}`);
    }
}

async function updateSecret(secretName: string, secretValue: string, env: Env): Promise<void> {
    const url = `https://api.cloudflare.com/client/v4/accounts/${env.CLOUDFLARE_ACCOUNT_ID}/workers/scripts/${env.CLOUDFLARE_WORKER_NAME}/secrets`;
    const body = JSON.stringify({
        name: secretName,
        text: secretValue,
        type: "secret_text"
    });

    const response = await fetch(url, {
        method: "PUT",
        headers: {
            "Content-Type": "application/json",
            "Authorization": `Bearer ${env.CLOUDFLARE_API_KEY}`
        },
        body: body
    });

    if (!response.ok) {
        throw new Error(`Failed to update secret: ${response.status} - ${response.statusText}`);
    }
}