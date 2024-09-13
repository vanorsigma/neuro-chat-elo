pub mod types;

use std::collections::HashMap;

use reqwest;
pub use tokio::sync::RwLock;
pub use types::*;

const DISCORD_PROFILE_URL: &str = "https://discord.com/api/v9/users/{user_id}/profile?with_mutual_guilds=false&with_mutual_friends=false&with_mutual_friends_count=false&guild_id=574720535888396288";
const DISCORD_AVATAR_URL: &str =
    "https://cdn.discordapp.com/avatars/{user_id}/{avatar}.webp?size=128";

pub struct DiscordClient {
    token: String,
    username_to_author_cache: RwLock<HashMap<String, DiscordAuthor>>,
}

impl DiscordClient {
    pub fn new(token: String) -> DiscordClient {
        Self {
            token,
            username_to_author_cache: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get_profile_for_user_id(
        &self,
        user_id: String,
    ) -> Result<DiscordProfileUserResponse, anyhow::Error> {
        Ok(reqwest::Client::new()
            .get(DISCORD_PROFILE_URL.replace("{user_id}", &user_id))
            .header("Authorization", self.token.to_string())
            .send()
            .await?
            .json::<DiscordProfileResponse>()
            .await?
            .user)
    }

    pub async fn set_username_author(&self, author: DiscordAuthor) {
        log::info!("setting username assoc for {}", author.name);
        self.username_to_author_cache
            .write()
            .await

            .insert(author.name.to_string(), author.clone());
    }

    pub async fn get_username_author(&self, username: String) -> Option<DiscordAuthor> {
        log::info!("get username assoc for {}", username);
        self.username_to_author_cache
            .read()
            .await
            .get(&username)
            .cloned()
    }
}

impl DiscordProfileUserResponse {
    pub fn get_profile_url(&self) -> String {
        DISCORD_AVATAR_URL
            .replace("{user_id}", &self.id)
            .replace("{avatar}", &self.avatar)
    }
}
