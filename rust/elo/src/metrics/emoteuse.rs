//! The emote metric
use std::{collections::HashMap, sync::Arc};

use crate::_types::clptypes::{Message, MetricUpdate};
use twitch_utils::seventvclient::SevenTVClient;

use super::metrictrait::AbstractMetric;

const WEIGHT_EMOTES: f32 = 1.0;

pub struct EmoteUse {
    seventv_client: Arc<SevenTVClient>,
}

impl EmoteUse {
    pub fn new(seventv_client: Arc<SevenTVClient>) -> Self {
        Self {
            seventv_client,
        }
    }
}

impl AbstractMetric for EmoteUse {
    fn can_parallelize(&self) -> bool {
        false
    }

    fn get_name(&self) -> String {
        String::from("emote_use")
    }

    fn get_metric(&mut self, message: Message, _sequence_no: u32) -> MetricUpdate {
        let update = match message {
            Message::Twitch(comment) => {
                MetricUpdate {
                    metric_name: self.get_name(),
                    updates: self.seventv_client
                        .get_emotes_in_comment(&comment)
                        .iter()
                        .fold(HashMap::new(), |mut acc, emote| {
                            *acc.entry(emote.id.clone()).or_insert(0.0) += WEIGHT_EMOTES;
                            acc
                        })
                }
            }
            _ => MetricUpdate::empty_with_name(self.get_name()), // TODO: discord emotes
        };
        update
    }
}
