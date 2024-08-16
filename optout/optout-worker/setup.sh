#!/bin/bash

source .env

echo "Registering commands with discord"
commands='[
  {
    "name": "opt_out",
    "description": "Opt out of discord leaderboards"
  },
  {
    "name": "opt_in",
    "description": "Opt back in to discord leaderboards"
  },
  {
    "name": "about",
    "description": "Get information about the leaderboards"
  }
]'

url="https://discord.com/api/v10/applications/$DISCORD_CLIENT_ID/commands"
response=$(curl -X PUT -H "Content-Type: application/json" -H "Authorization: Bot $DISCORD_CLIENT_SECRET" -d "$commands" "$url")

if [ $? -eq 0 ]; then
  echo "Registered all discord commands"
  data=$(echo "$response" | jq .)
  echo "$data"
else
  echo "Error registering commands"
  errorText="Error registering commands \n $url: $(echo "$response" | jq -r .status) $(echo "$response" | jq -r .statusText)"
  error=$(echo "$response" | jq -r .)
  if [ "$error" != "null" ]; then
    errorText="$errorText \n\n $error"
  fi
  echo "$errorText"
fi
