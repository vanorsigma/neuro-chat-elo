# Cloudflare Optout Worker

The Cloudflare optout worker runs on webhook triggers from Twitch Whispers and Discord Command Interactions. Optouts are stored in a Firebase Firestore for retrieval by the rust processor.

## Usage

### Setup

Copy .env.template and rename it to .env

#### Env Variables

The following values should be added to your .env file. Values marked with a `*` should also be added to your wrangler.toml (the star should not be included)

##### Cloudflare

- CLOUDFLARE_API_KEY: Found in My Profile -> API Tokens, only needs Workers Scripts edit permission
- CLOUDFLARE_ACCOUNT_ID: Found in Workers & Pages -> Overview
- CLOUDFLARE_WORKER_NAME\*: Defined in wrangler.toml, defaults to "optout-worker"

##### Twitch API

The Twitch bot needs to be associated with a Twitch Applicaton and a bot account. After creating an account, go to the [Twitch Developer Console](https://dev.twitch.tv/console) and create an application

- TWITCH_CLIENT_ID: Found in the Application menu of the Twitch Dev Console
- TWITCH_CLIENT_SECRET: Found in the Application menu of the Twitch Dev Console
- TWITCH_WEBHOOK_SECRET: A value of your choosing, used to verify Twitch webhook requests
- TWITCH_BOT_USERNAME\*: The name of your Twitch bot account
- TWITCH_BOT_ID\*: The ID of your Twitch bot account

##### Discord

The Discord bot needs to be associated with a Discord application. Create a new application in the [Discord Developer Console](https://discord.com/developers/applications)

- DISCORD_APPLICATION_ID: Found in your Application -> General Information
- DISCORD_BOT_TOKEN: Found in your your Application -> Bot -> Reset Token
- DISCORD_PUBLIC_KEY\*: Found in your Application -> General Information

##### Firebase

You need a Firebase Firestore to store opted out users. Create a new Firebase project and Firestore Database from the [Firebase Console](https://console.firebase.google.com/)

- FIREBASE_CREDS: Create a new service account and download the private key in Project settings -> Generate new private key. Paste the JSON as a single line in this env variable
- FIREBASE_PROJECT_ID\*: The ID of the project you just created
