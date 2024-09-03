// TODO: Add proper error handling
import { addOptOut, removeOptOut } from './firebase';
import { updateSecret } from './cloudflareHelpers';
import { TwitchAuthFailureError, TwitchRequestFailureError, UnknownCommandError } from './errors';
import crypto from 'node:crypto';
import Buffer from 'node:buffer';
import { RefreshingAuthProvider } from '@twurple/auth';
import { ApiClient } from '@twurple/api';
import { EventSubBase, EventSubUserWhisperMessageEventData } from '@twurple/eventsub-base';
import { get } from 'fireworkers';

interface TokenData {
    accessToken: string;
    refreshToken: string;
    expiresIn: number;
    obtainmentTimestamp: number;
}

function getUserTokenData(env: Env): TokenData {
    const expiresIn = env.TWITCH_USER_EXPIRES_IN ? parseInt(env.TWITCH_USER_EXPIRES_IN) : 0;
    const obtainmentTimestamp = env.TWITCH_USER_OBTAIN_TIMESTAMP ? parseInt(env.TWITCH_USER_OBTAIN_TIMESTAMP) : 0;

    return {
        accessToken: env.TWITCH_USER_AUTH,
        refreshToken: env.TWITCH_REFRESH_TOKEN,
        expiresIn: expiresIn,
        obtainmentTimestamp: obtainmentTimestamp,
    };
}

export function getTwitchAuthProvider(env: Env): RefreshingAuthProvider {
    let auth_provider = new RefreshingAuthProvider({
        clientId: env.TWITCH_CLIENT_ID,
        clientSecret: env.TWITCH_CLIENT_SECRET,
    });

    auth_provider.onRefresh(async (_userId: String, newTokenData: TokenData) => {
        await updateSecret('TWITCH_USER_AUTH', newTokenData.accessToken, env);
        await updateSecret('TWITCH_REFRESH_TOKEN', newTokenData.refreshToken, env);
        await updateSecret('TWITCH_USER_EXPIRES_IN', newTokenData.expiresIn.toString(), env);
        await updateSecret('TWITCH_USER_OBTAIN_TIMESTAMP', newTokenData.obtainmentTimestamp.toString(), env);
    });

    let tokenData = getUserTokenData(env);
    auth_provider.addUser(env.TWITCH_BOT_ID, tokenData);

    return auth_provider;
}

export function getTwitchEventSub(env: Env): EventSubBase {
    let eventSub = new EventSubBase({
        apiClient: new ApiClient({ authProvider: getTwitchAuthProvider(env) }),
        hostName: env.CLOUDFLARE_WORKER_URL,
        pathPrefix: '/twitch',
        secret: env.TWITCH_WEBHOOK_SECRET,
    });
    eventSub.onUserWhisperMessage(env.TWITCH_BOT_ID, async (data: EventSubUserWhisperMessageEventData) => {
        let command = data.whisper.text.split(' ')[0];
        let args = data.whisper.text.split(' ').slice(1);
        if (command === 'optout') {
            await addOptOut(data.from_user_id, 'twitch', env);
            // await message.reply('You have been opted out of the leaderboard.');
        } else if (command === 'optin') {
            await removeOptOut(data.from_user_id, 'twitch', env);
            // await message.reply('You have been opted back into the leaderboard.');
        } else {
            throw new UnknownCommandError('Unknown command', 'Unknown command');
        }
    });
    return eventSub;
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

async function updateTwitchSecrets(env: Env): Promise<{ accessToken: string; refreshToken: string }> {
    const { accessToken, refreshToken } = await refreshTwitchToken(
        env.TWITCH_CLIENT_ID,
        env.TWITCH_CLIENT_SECRET,
        env.TWITCH_REFRESH_TOKEN,
    );
    await updateSecret('TWITCH_USER_AUTH', accessToken, env);
    await updateSecret('TWITCH_REFRESH_TOKEN', refreshToken, env);
    return { accessToken, refreshToken };
}
