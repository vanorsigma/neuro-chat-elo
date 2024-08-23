import { verifyTwitch, handleTwitchNotification, handleTwitchRevocation, handleTwitchVerification } from './twitch';
import { InvalidSignatureError } from './errors';

export default {
    async fetch(request: Request, env: Env, ctx: ExecutionContext): Promise<Response> {
        const url = new URL(request.url);

        switch (url.pathname) {
            case '/discord':
                return await new Response('Hello Discord!');
            case '/twitch':
                return await preHandleTwitch(request, env);
            default:
                return new Response('route not found', { status: 404 });
        }
    },
} satisfies ExportedHandler<Env>;

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
