/**
 * Welcome to Cloudflare Workers! This is your first worker.
 *
 * - Run `npm run dev` in your terminal to start a development server
 * - Open a browser tab at http://localhost:8787/ to see your worker in action
 * - Run `npm run deploy` to publish your worker
 *
 * Bind resources to your worker in `wrangler.toml`. After adding bindings, a type definition for the
 * `Env` object can be regenerated with `npm run cf-typegen`.
 *
 * Learn more at https://developers.cloudflare.com/workers/
 */

import { handleWhisper, verifyTwitch } from "./twitch";
import { TwitchNotification } from "./whisperMessage";

export default {
	async fetch(request, env, ctx): Promise<Response> {
		const url = new URL(request.url);

		switch (url.pathname) {
			case "/discord":
				return new Response("discord request");
			case "/twitch":
				return await preHandleTwitch(request, env, ctx);
			default:
				return new Response("route not found", { status: 404 });
		}
	},
} satisfies ExportedHandler<Env>;

async function preHandleTwitch(request: Request<unknown, IncomingRequestCfProperties>, env: Env, ctx: ExecutionContext): Promise<Response> {
	const body = await request.text();
	const valid = verifyTwitch(env.TWITCH_WEBHOOK_SECRET, request.headers, body);

	if (!valid) {
		return new Response("Invalid signature", { status: 403 });
	}

	const messageType = request.headers.get("Twitch-Eventsub-Message-Type");
	switch (messageType) {
		case "notification":
			console.log(`Received notification: ${body}`);
			return await handleTwitchNotification(request, body, env);
		case "webhook_callback_verification":
			console.log(`Performing webhook callback verification`);
			return await handleTwitchVerification(body);
		case "revocation":
			console.log("Twitch subscription was revoked");
			return await handleTwitchRevocation();
		default:
			console.log(`Unknown message type: ${messageType}`);
			return new Response("Unknown message type", { status: 500 });
	}
}

async function handleTwitchNotification(request: Request, body: string, env: Env): Promise<Response> {
	console.log(`Processing Twitch notification with body: ${body}`);

	const data: TwitchNotification = JSON.parse(body);

	const response = await handleWhisper(data.event, env);

	return new Response("Notification received", { status: 200 });
}

async function handleTwitchVerification(body: string): Promise<Response> {
	const bodyJson = JSON.parse(body);
	const challenge = bodyJson["challenge"] as string;

	return new Response(challenge, {
		headers: {
			"Content-Type": challenge.length.toString(),
		},
		status: 200
	});
}

async function handleTwitchRevocation(): Promise<Response> {
	console.log("Handling revocation");

	return new Response(null, { status: 204 });
}
