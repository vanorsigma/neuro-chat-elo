use elo::_types::clptypes::Message;
use gcp_auth::{CustomServiceAccount, TokenProvider};
use log::{info, warn};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Debug, Deserialize)]
struct FirebaseResponse {
    documents: Vec<Document>,
}

#[derive(Debug, Deserialize)]
struct Document {
    fields: DocumentFields,
}

#[derive(Debug, Deserialize)]
struct DocumentFields {
    id: StringValue,
    platform: StringValue,
}

#[derive(Debug, Deserialize)]
struct StringValue {
    #[serde(rename = "stringValue")]
    string_value: String,
}

pub struct OptOutManager {
    pub service_account: CustomServiceAccount,
    pub twitch_ids: HashSet<String>,
    pub discord_ids: HashSet<String>,
}

impl OptOutManager {
    /// Create a new OptOutList from a JSON service account key.
    ///
    /// # Arguments
    /// * `json_creds` - A string of valid JSON containing the full service account credentials.
    pub fn new(json_creds: String) -> Result<Self, Box<dyn std::error::Error>> {
        let service_account = CustomServiceAccount::from_json(json_creds.as_str())?;

        let twitch_names = HashSet::new();
        let discord_names = HashSet::new();

        Ok(Self {
            service_account,
            twitch_ids: twitch_names,
            discord_ids: discord_names,
        })
    }

    /// Refresh the stored opt-out lists from Firestore. To be called periodically during live elo.
    pub async fn refresh_optouts(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match self.get_optouts().await {
            Ok((twitch_names, discord_names)) => {
                self.twitch_ids = twitch_names;
                self.discord_ids = discord_names;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    async fn get_optouts(
        &self,
    ) -> Result<(HashSet<String>, HashSet<String>), Box<dyn std::error::Error>> {
        let client = Client::new();
        let response: FirebaseResponse = serde_json::from_str(client
            .get("https://firestore.googleapis.com/v1/projects/neuro-chat-elo/databases/(default)/documents/opt_outs/")
            .header("Authorization", format!("Bearer {}", self.service_account.token(&["https://www.googleapis.com/auth/cloud-platform"]).await?.as_str()))
            .send()
            .await?
            .text()
            .await?
            .as_str())?;

        let mut twitch_names = HashSet::new();
        let mut discord_names = HashSet::new();

        for document in response.documents {
            match document.fields.platform.string_value.as_str() {
                "twitch" => {
                    twitch_names.insert(document.fields.id.string_value.clone());
                }
                "discord" => {
                    discord_names.insert(document.fields.id.string_value.clone());
                }
                _ => warn!("Unknown platform found in opt-out list"),
            }
        }

        info!(
            "Opt-out list refreshed with {} Twitch names and {} Discord names",
            twitch_names.len(),
            discord_names.len()
        );

        Ok((twitch_names, discord_names))
    }

    /// Check if a message is from an opted-out user.
    ///
    /// # Arguments
    /// * `message` - The message to check.
    pub fn is_opted_out(&self, message: &Message) -> bool {
        match message {
            Message::Twitch(comment) => self.twitch_ids.contains(&comment.commenter._id),
            Message::Discord(message) => self.discord_ids.contains(&message.author.id),
            _ => false,
        }
    }
}
