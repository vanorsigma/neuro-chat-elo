use elo::_types::clptypes::Message;
use log::info;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashSet;
use std::env;

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
#[allow(non_snake_case)]
struct StringValue {
    stringValue: String,
}

pub struct OptOutList {
    pub twitch_ids: HashSet<String>,
    pub _discord_ids: HashSet<String>,
}

impl OptOutList {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let token = env::var("FIRESTORE_TOKEN")?;
        let client = Client::new();
        let url = "https://firestore.googleapis.com/v1/projects/neuro-chat-elo/databases/(default)/documents/opt_outs/";

        let res = client.get(url).bearer_auth(token).send().await?;

        let body = res.text().await?;
        let parsed = serde_json::from_str::<Value>(&body)?;

        // println!("{:?}", parsed);

        let binding = vec![];
        let documents = parsed["documents"].as_array().unwrap_or(&binding);

        println!("{:?}", documents);

        let mut twitch_names = HashSet::new();
        let mut discord_names = HashSet::new();

        for doc in documents {
            if let Ok(document) = serde_json::from_value::<Document>(doc.clone()) {
                match &document.fields.platform.stringValue as &str {
                    "twitch" => {
                        twitch_names.insert(document.fields.id.stringValue);
                    }
                    "discord" => {
                        discord_names.insert(document.fields.id.stringValue);
                    }
                    _ => {
                        println!(
                            "Unknown platform: {:?}",
                            document.fields.platform.stringValue
                        );
                    }
                }
            }
        }

        info!(
            "Opt-out list loaded with {} Twitch IDs and {} Discord IDs",
            twitch_names.len(),
            discord_names.len()
        );

        Ok(Self {
            twitch_ids: twitch_names,
            _discord_ids: discord_names,
        })
    }

    pub fn is_opted_out(&self, message: &Message) -> bool {
        match message {
            Message::Twitch(comment) => self.twitch_ids.contains(&comment.commenter._id),
        }
    }
}
