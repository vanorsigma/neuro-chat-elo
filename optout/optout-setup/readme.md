## steps
### twitch
1. Get app id and client secret
2. construct `twitchAPI.oauth.UserAuthenticator`
3. authenticate user with `AuthScope.WHISPERS_READ`
4. subscribe to event using normal twitch api w/ app id and client secret (no use for user tokens)