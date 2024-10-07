pub mod types;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter},
};

use anyhow::Error;
use reqwest::{self, StatusCode};
pub use tokio::sync::RwLock;
pub use types::*;

const DISCORD_PROFILE_URL: &str = "https://discord.com/api/v9/users/{user_id}/profile?with_mutual_guilds=false&with_mutual_friends=false&with_mutual_friends_count=false&guild_id=574720535888396288";
const DISCORD_AVATAR_URL: &str =
    "https://cdn.discordapp.com/avatars/{user_id}/{avatar}.webp?size=128";

pub struct DiscordClient {
    token: String,
    username_to_author_cache: RwLock<HashMap<String, DiscordAuthor>>,
    uid_to_author_cache: RwLock<HashMap<String, DiscordAuthor>>,
}

impl DiscordClient {
    pub fn new(token: String) -> DiscordClient {
        Self {
            token,
            username_to_author_cache: RwLock::new(HashMap::new()),
            uid_to_author_cache: RwLock::new(HashMap::new()),
        }
    }

    pub async fn fetch_new_profile_only_if_not_found(
        &self,
        author: DiscordAuthor,
    ) -> Result<DiscordAuthor, anyhow::Error> {
        if let StatusCode::NOT_FOUND = reqwest::Client::new()
            .get(author.avatar_url.clone())
            .send()
            .await?
            .status()
        {
            let profile = self.get_profile_for_user_id(author.id).await?;
            Ok(DiscordAuthor {
                id: profile.id,
                name: author.name,
                nickname: author.nickname,
                roles: author.roles,
                avatar_url: profile.avatar,
            })
        } else {
            Ok(author)
        }
    }

    pub async fn get_profile_for_user_id(
        &self,
        user_id: String,
    ) -> Result<TransformedDiscordProfileUserResponse, anyhow::Error> {
        Ok(reqwest::Client::new()
            .get(DISCORD_PROFILE_URL.replace("{user_id}", &user_id))
            .header("Authorization", self.token.to_string())
            .send()
            .await?
            .json::<DiscordProfileResponse>()
            .await?
            .user
            .into())
    }

    pub async fn cached_get_profile_for_user_id(
        &self,
        user_id: String,
    ) -> Result<TransformedDiscordProfileUserResponse, anyhow::Error> {
        if let Some(author) = self.get_userid_author(user_id.clone()).await {
            return Ok(author.into());
        }

        log::debug!("cache miss for {user_id}");
        self.get_profile_for_user_id(user_id).await
    }

    pub async fn preload_cache(&self, cache_path: &str) -> Result<(), Error> {
        futures::future::join_all(
            serde_json::from_reader::<_, Vec<DiscordAuthor>>(BufReader::new(File::open(
                &cache_path,
            )?))?
            .into_iter()
            .map(|author| async {
                self.set_author_cache(
                    self.fetch_new_profile_only_if_not_found(author.clone())
                        .await
                        .unwrap_or(author),
                )
                .await
            }),
        )
        .await;

        Ok(())
    }

    pub async fn dump_cache(&self, cache_path: &str) -> Result<(), Error> {
        serde_json::to_writer(
            BufWriter::new(File::create(&cache_path)?),
            &self
                .username_to_author_cache
                .read()
                .await
                .values()
                .cloned()
                .collect::<Vec<_>>(),
        )?;
        Ok(())
    }

    pub async fn set_author_cache(&self, author: DiscordAuthor) {
        self.username_to_author_cache
            .write()
            .await
            .insert(author.name.to_string(), author.clone());

        self.uid_to_author_cache
            .write()
            .await
            .insert(author.id.to_string(), author);
    }

    pub async fn get_username_author(&self, username: String) -> Option<DiscordAuthor> {
        self.username_to_author_cache
            .read()
            .await
            .get(&username)
            .cloned()
    }

    pub async fn get_userid_author(&self, uid: String) -> Option<DiscordAuthor> {
        self.uid_to_author_cache.read().await.get(&uid).cloned()
    }
}

impl DiscordProfileUserResponse {
    pub fn get_profile_url(&self) -> String {
        DISCORD_AVATAR_URL
            .replace("{user_id}", &self.id)
            .replace("{avatar}", &self.avatar)
    }
}

pub struct TransformedDiscordProfileUserResponse {
    pub id: String,
    pub avatar: String,
    pub global_name: String,
}

impl Into<TransformedDiscordProfileUserResponse> for DiscordProfileUserResponse {
    fn into(self) -> TransformedDiscordProfileUserResponse {
        let url = self.get_profile_url();
        TransformedDiscordProfileUserResponse {
            id: self.id,
            avatar: url,
            global_name: self.global_name,
        }
    }
}

impl Into<TransformedDiscordProfileUserResponse> for DiscordAuthor {
    fn into(self) -> TransformedDiscordProfileUserResponse {
        TransformedDiscordProfileUserResponse {
            id: self.id,
            avatar: self.avatar_url,
            global_name: self.nickname,
        }
    }
}
