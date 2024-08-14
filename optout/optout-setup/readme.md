## steps

### Twitch

1. Get app id and client secret
2. Construct `twitchAPI.oauth.UserAuthenticator`
3. Authenticate user with `AuthScope.WHISPERS_READ`
4. Subscribe to event using normal twitch api w/ app id and client secret
5. Show user auth token for sending whispers
6. Add user auth token to cloudflare worker as TWITCH_USER_AUTH

### Discord

1. Get Discord token
