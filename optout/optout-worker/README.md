# Cloudflare Optout Worker

The Cloudflare optout worker runs on webhook triggers from Twitch Whispers and Discord Command Interactions. Optouts are stored in a Firebase Firestore for retrieval by the rust processor.

## Usage

### Setup

The Twitch bot needs to be associated with a Twitch Applicaton and a bot account. After creating an account, go to the [Twitch Developer Console](https://dev.twitch.tv/console) and create an application. The OAuth Redirect URL must be set to `http://localhost:17563` for the setup process, but can be changed afterwards.

The Discord bot needs to be associated with a Discord application. Create a new application in the [Discord Developer Console](https://discord.com/developers/applications)

You need a Firebase Firestore to store opted out users. Create a new Firebase project and Firestore Database from the [Firebase Console](https://console.firebase.google.com/)

#### Configure Wrangler.toml

You need to add the following env variables to the wrangler.toml:

- CLOUDFLARE_WORKER_NAME: Defined in wrangler.toml, defaults to "optout-worker"
- FIREBASE_PROJECT_ID: The ID of the project you just created
- TWITCH_BOT_USERNAME: The name of your Twitch bot account
- TWITCH_BOT_ID: The ID of your Twitch bot account
- DISCORD_APPLICATION_ID: Found in your Application -> General Information
- DISCORD_PUBLIC_KEY: Found in your Application -> General Information

#### Deploy the worker

The worker needs to be deployed so that you can get its url. To do so, run:

```bash
npx wrangler deploy
```

And copy the url it provides. This url should end with `.workers.dev`

#### Configure setup tool

Copy the .env.template in the optout-setup folder and rename to .env
The following values should be added to it:

##### Cloudflare

- CLOUDFLARE_API_KEY: Found in My Profile -> API Tokens, only needs Workers Scripts edit permission
- CLOUDFLARE_ACCOUNT_ID: Found in Workers & Pages -> Overview
- CLOUDFLARE_WORKER_NAME: Defined in wrangler.toml, defaults to "optout-worker"
- CLOUDFLARE_WORKER_URL: Your cloudflare worker's trigger url. Must iunclude https://

##### Twitch API

- TWITCH_CLIENT_ID: Found in the Application menu of the Twitch Dev Console
- TWITCH_CLIENT_SECRET: Found in the Application menu of the Twitch Dev Console
- TWITCH_WEBHOOK_SECRET: A value of your choosing, used to verify Twitch webhook requests
- TWITCH_BOT_USERNAME: The name of your Twitch bot account
- TWITCH_BOT_ID: The ID of your Twitch bot account

##### Discord

- DISCORD_APPLICATION_ID: Found in your Application -> General Information
- DISCORD_BOT_TOKEN: Found in your your Application -> Bot -> Reset Token
- DISCORD_PUBLIC_KEY: Found in your Application -> General Information

##### Firebase

- FIREBASE_CREDS: Create a new service account and download the private key in Project settings -> Generate new private key. Paste the JSON as a single line in this env variable
- FIREBASE_PROJECT_ID: The ID of the project you just created

Copy .env.template and rename it to .env

#### Run the setup tool

You must be logged into your Twitch bot account in your browser so that the setup process can pull yor auth tokens.
Once the env has been configured, simply run main.py and wait for it to complete. The the Twitch side of your bot should now be fuly working.

#### Configure Discord Interactions

Once the bot is deployed and set up, the last step is to set your Discord Interactions Endpoint.
Navigate to your application's General Information, and paste your worker's url with `/discord` added to the end into the Interactions Endpoint URL.

#### Successive deployments

Future deployments should be done by running the following:

```bash
npx wrangler deploy
```
