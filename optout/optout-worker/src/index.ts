import {
    verifyTwitch,
    handleTwitchNotification,
    handleTwitchRevocation,
    handleTwitchVerification,
} from './twitch/twitch';
import { InvalidSignatureError } from './errors';
import { AutoRouter } from 'itty-router'; // ~1kB
import { RefreshingAuthProvider } from './twitch/twurple/packages/auth/src';
import { ApiClient } from './twitch/twurple/packages/api/src';
import { EventSubBase, EventSubUserWhisperMessageEvent } from './twitch/twurple/packages/eventsub-base/src';
import { get } from 'fireworkers';
import { getTwitchEventSub } from './twitch/twitchV2';

const router = AutoRouter();

router.get('/discord', async (request: Request<unknown, IncomingRequestCfProperties>, env: Env) => {
    console.log(EventSubUserWhisperMessageEvent);
    return new Response('Hello Discord!');
});

router.get('/twitch', async (request: Request<unknown, IncomingRequestCfProperties>, env: Env) => {
    let eventSub = getTwitchEventSub(env);
    eventSub.apply(router as any);
    return await preHandleTwitch(request, env);
});

router.get('/test', async (request: Request<unknown, IncomingRequestCfProperties>, env: Env) => {
    console.log(EventSubUserWhisperMessageEvent);
    return new Response('Hello test!');
});

export default router;

// const handler = {
//     async fetch(request: Request, env: Env, ctx: ExecutionContext): Promise<Response> {
//         const url = new URL(request.url);

//         switch (url.pathname) {
//             case '/discord':
//                 return await new Response('Hello Discord!');
//             case '/twitch':
//                 return await preHandleTwitch(request, env);
//             default:
//                 return new Response('route not found', { status: 404 });
//         }
//     },
// } satisfies ExportedHandler<Env>;

async function preHandleTwitch(request: Request<unknown, IncomingRequestCfProperties>, env: Env): Promise<Response> {
    const body = await request.text();
    const valid = verifyTwitch(env.TWITCH_WEBHOOK_SECRET, request.headers, body);

    if (!valid) {
        console.warn('Invalid signature for Twitch request');
        throw new InvalidSignatureError('Invalid signature for Twitch request', null);
    }
    console.log(`Verified Twitch request`);

    const messageType = request.headers.get('Twitch-Eventsub-Message-Type');
    switch (messageType) {
        case 'notification':
            console.log(`Received notification: ${body}`);
            return await handleTwitchNotification(request, body, env);
        case 'webhook_callback_verification':
            console.log(`Performing webhook callback verification`);
            return await handleTwitchVerification(body);
        case 'revocation':
            console.log('Twitch subscription was revoked');
            return await handleTwitchRevocation();
        default:
            console.log(`Unknown message type: ${messageType}`);
            return new Response('Unknown message type', { status: 500 });
    }
}
