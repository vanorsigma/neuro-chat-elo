//! The emote metric
use core::panic;
use std::sync::Arc;

use crate::_types::clptypes::{Message, MetricUpdate};
use twitch_utils::seventvclient::SevenTVClient;

use super::metrictrait::AbstractMetric;

const WEIGHT_EMOTES: f32 = 0.02;

pub struct Emote {
    seventv_client: Option<Arc<SevenTVClient>>,
}

impl Emote {
    pub fn set_seventv_client(&mut self, client: Arc<SevenTVClient>) {
        self.seventv_client = Some(client);
    }
}

impl AbstractMetric for Emote {
    async fn new() -> Self {
        Self {
            seventv_client: None,
        }
    }

    fn can_parallelize(&self) -> bool {
        false
    }

    fn get_name(&self) -> String {
        String::from("emote")
    }

    fn get_metric(&mut self, message: Message, _sequence_no: u32) -> MetricUpdate {
        match message {
            Message::Twitch(comment) => {
                let score: f32 = self
                    .seventv_client
                    .as_ref()
                    .unwrap_or_else(|| panic!("7TV client not set"))
                    .get_emotes_in_comment(&comment)
                    .len() as f32
                    * WEIGHT_EMOTES;
                self.twitch_comment_shortcut(comment, score)
            }
            _ => MetricUpdate::empty_with_name(self.get_name()), // TODO: discord emotes
        }
    }
}
