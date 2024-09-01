use std::collections::HashMap;

use adventures_utils::{AdventuresGetLeaderboardData, AdventuresGetLeaderboardRequest, AdventuresGetLeaderboardType, AdventuresMetadataRequest, AdventuresMetadataResponse, AdventuresRankItem, AdventuresRankItemWithAvatar};
use discord_utils::DiscordClient;
use itertools::Itertools;
use reqwest;
const ADVENTURES_LEADERBOARD_URL: &str = "https://rants.theharrisontemple.com:8727/leaderboard";

pub struct AdventuresDownloaderProxy {
    client: DiscordClient,
}

impl AdventuresDownloaderProxy {
    pub fn new(token: String) -> Self {
        Self {
            client: DiscordClient::new(token),
        }
    }

    async fn get_rank_item_with_avatar(&self, rank_item: AdventuresRankItem) -> Result<AdventuresRankItemWithAvatar, anyhow::Error> {
        let avatar_url = self.client.get_profile_for_user_id(rank_item.uid.clone()).await?.get_profile_url();
        AdventuresRankItemWithAvatar::with_adventures_rank_item(rank_item, avatar_url)
    }

    async fn get_rank_items_with_avatar(&self, rank_items: Vec<AdventuresRankItem>) -> Result<Vec<AdventuresRankItemWithAvatar>, anyhow::Error> {
        futures::future::try_join_all(rank_items.into_iter().map(|rank_item| self.get_rank_item_with_avatar(rank_item))).await
    }

    fn get_highest_score_rank_items(&self, mut rank_items: Vec<AdventuresRankItem>) -> Vec<AdventuresRankItem> {
        rank_items.sort_by_key(|rank_item| u64::from_str_radix(&rank_item.score, 10).unwrap());
        rank_items.reverse();
        rank_items.into_iter().unique_by(|rank_item| rank_item.uid.clone()).collect()
    }

    pub async fn get_ranks(
        &self,
    ) -> Result<HashMap<String, Vec<AdventuresRankItemWithAvatar>>, anyhow::Error> {
        let metadata_response = reqwest::Client::new()
            .post(ADVENTURES_LEADERBOARD_URL)
            .json(&AdventuresMetadataRequest::default())
            .send()
            .await?
            .json::<AdventuresMetadataResponse>()
            .await?;

        let version = metadata_response
            .versions
            .first()
            .ok_or(anyhow::anyhow!("cannot get metadata versions"))?;

        let responses =
            futures::future::try_join_all(version.maps.clone().into_iter().map(|game_map| async {
                reqwest::Client::new()
                    .post(ADVENTURES_LEADERBOARD_URL)
                    .json(&AdventuresGetLeaderboardRequest {
                        ty: AdventuresGetLeaderboardType::GetLeaderboard,
                        data: AdventuresGetLeaderboardData {
                            version: version.version_number.clone(),
                            version_maps: version.maps.clone(),
                            map: game_map.to_string(),
                        },
                    })
                    .send()
                    .await
                    .map(|response| (game_map, response))
            }))
            .await?;

        let responses_with_avatar = futures::future::try_join_all(responses.into_iter().map(
            |(game_map, response)| async {
                response
                    .json::<Vec<AdventuresRankItem>>()
                    .await
                    .map(|rank_items| (game_map, self.get_rank_items_with_avatar(self.get_highest_score_rank_items(rank_items))))
            },
        ))
            .await?;

        Ok(
            futures::future::try_join_all(responses_with_avatar.into_iter().map(
                |(game_map, response)| async {
                    response
                        .await
                        .map(|rank_item| (game_map, rank_item))
                },
            ))
            .await?
            .into_iter()
            .collect()
        )
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn magical_test() {
        let token = std::env::var("CHAT_DISCORD_TOKEN").unwrap();
        let ranks = super::AdventuresDownloaderProxy::new(token).get_ranks().await;
        println!("{:#?}", ranks.unwrap());
    }
}
