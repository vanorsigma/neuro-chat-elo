// TODO: Add proper error handling
import { addOptOut, removeOptOut } from './firebase';
import { updateSecret } from './cloudflareHelpers';
import { TwitchAuthFailureError, TwitchRequestFailureError, UnknownCommandError } from './errors';
import crypto from 'node:crypto';
import Buffer from 'node:buffer';

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

function getHmacMessage(headers: Headers, body: string) {
    return headers[TWITCH_MESSAGE_ID] + headers[TWITCH_MESSAGE_TIMESTAMP] + body;
}

function getHmac(secret, message) {
    return crypto.createHmac('sha256', secret).update(message).digest('hex');
}

function verifyMessage(hmac, verifySignature) {
    return crypto.timingSafeEqual(Buffer.from(hmac), Buffer.from(verifySignature));
}

export async function verifyTwitch(
    twitchWebhookSecret: string,
    headers: Headers,
    requestBody: string,
): Promise<boolean> {
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

export async function handleTwitchNotification(request: Request, body: string, env: Env): Promise<Response> {
    console.log(`Processing Twitch notification with body: ${body}`);

    const data: TwitchNotification = JSON.parse(body);
    await handleWhisper(data.event, env);

    return new Response('Notification received');
}

export async function handleTwitchVerification(body: string): Promise<Response> {
    const bodyJson = JSON.parse(body);
    const challenge = bodyJson['challenge'] as string;

    return new Response(challenge, {
        headers: {
            'Content-Type': challenge.length.toString(),
        },
        status: 200,
    });
}

export async function handleTwitchRevocation(): Promise<Response> {
    console.warn('Handling revocation, but why tho?');

    return new Response(null, { status: 204 });
}

async function handleWhisper(event: WhisperEvent, env: Env): Promise<Response> {
    const text = event.whisper.text;
    const user = event.from_user_name;
    const userId = event.from_user_id;

    console.log(`Received whisper from ${user} (${userId}): ${text}`);

    switch (text) {
        case '/opt_out':
            await handleOptout(userId, env);
            return new Response('Success');
        case '/opt_in':
            await handleOptin(userId, env);
            return new Response('Success');
        default:
            await sendWhisper(userId, 'Thank you for checking out the Neuro Chat Elo Leaderboards! If you want to opt out send me /opt_out, and to opt back in send /opt_in', env);
            return new Response('Success');
    }
}

async function handleOptout(userId: string, env: Env) {
    console.log(`Opting out ${userId}`);
    const result = await addOptOut(userId, 'twitch', env);
    try {
        await sendWhisper(userId, 'You have been opted out of the leaderboards', env);
    } catch (error) {
        if (error instanceof TwitchAuthFailureError) {
            await updateTwitchAndWhisper(userId, 'You have been opted out of the leaderboards', env);
        } else {
            throw error;
        }
    }
}

async function handleOptin(userId: string, env: Env) {
    console.log(`Opting in ${userId}`);
    const result = await removeOptOut(userId, 'twitch', env);
    try {
        await sendWhisper(userId, 'You have been opted back into the leaderboards', env);
    } catch (error) {
        if (error instanceof TwitchAuthFailureError) {
            await updateTwitchAndWhisper(userId, 'You have been opted back into the leaderboards', env);
        } else {
            throw error;
        }
    }
}

async function sendWhisper(userId: string, text: string, env: Env) {
    const url = `https://api.twitch.tv/helix/whispers?from_user_id=${env.TWITCH_BOT_ID}&to_user_id=${userId}&message=${encodeURIComponent(text)}`;
    const response = await fetch(url, {
        method: 'POST',
        headers: {
            'Client-ID': env.TWITCH_CLIENT_ID,
            Authorization: `Bearer ${env.TWITCH_USER_AUTH}`,
        },
    });

    if (response.status == 401 || response.status == 403) {
        throw new TwitchAuthFailureError('Twith Whisper auth failed', await response.json());
    } else if (!response.ok) {
        throw new TwitchRequestFailureError('Could not send whisper', await response.json());
    }
}

/**
 * Refreshes a Twitch user token using the refresh token.
 *
 * @param twitchClientId - The Twitch client ID.
 * @param twitchClientSecret - The Twitch client secret.
 * @param refreshToken - The latest user auth refresh token.
 * @returns A promise that resolves to an object containing the new access and refresh tokens.
 */
async function refreshTwitchToken(
    twitchClientId: string,
    twitchClientSecret: string,
    refreshToken: string,
): Promise<{ accessToken: string; refreshToken: string }> {
    const url = 'https://id.twitch.tv/oauth2/token';
    const body = new URLSearchParams({
        grant_type: 'refresh_token',
        refresh_token: refreshToken,
        client_id: twitchClientId,
        client_secret: twitchClientSecret,
    });

    const response = await fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded',
        },
        body: body.toString(),
    });

    const data = await response.json();
    if (response.ok) {
        return { accessToken: data.access_token, refreshToken: data.refresh_token };
    } else {
        throw new TwitchAuthFailureError('Could not refresh Twitch token', data);
    }
}

async function updateTwitchSecrets(env: Env) {
    const { accessToken, refreshToken } = await refreshTwitchToken(
        env.TWITCH_CLIENT_ID,
        env.TWITCH_CLIENT_SECRET,
        env.TWITCH_REFRESH_TOKEN,
    );
    await updateSecret('TWITCH_USER_AUTH', accessToken, env);
    await updateSecret('TWITCH_REFRESH_TOKEN', refreshToken, env);
}

/**
 * Updates the Twitch user auth token and sends a whisper to the user before updating the secrets.
 * bot updateSecret calls can take a while to propagate, so we want to make sure the whisper is sent first.
 * @param userId - The user ID.
 * @param text - The whisper text.
 * @param env - The worker's environment variables.
 * @returns A promise that resolves when the whisper has been sent and the secrets have been updated.
 */
async function updateTwitchAndWhisper(userId: string, text: string, env: Env) {
    const { accessToken, refreshToken } = await refreshTwitchToken(
        env.TWITCH_CLIENT_ID,
        env.TWITCH_CLIENT_SECRET,
        env.TWITCH_REFRESH_TOKEN,
    );
    env.TWITCH_USER_AUTH = accessToken;
    env.TWITCH_REFRESH_TOKEN = refreshToken;
    await sendWhisper(userId, text, env);
    await updateSecret('TWITCH_USER_AUTH', accessToken, env);
    await updateSecret('TWITCH_REFRESH_TOKEN', refreshToken, env);
}
