use std::collections::HashMap;
use std::sync::Arc;

use twitch_utils::TwitchAPIWrapper;

use crate::_types::clptypes::{Message, MetricUpdate};
use crate::metrics::metrictrait::AbstractMetric;

const IRONMOUSE_NEURO_FACTION: u64 = 1;

pub struct Score {
    twitch: Arc<TwitchAPIWrapper>,
}

impl Score {
    pub fn new(twitch: Arc<TwitchAPIWrapper>) -> Self {
        Self { twitch }
    }
}

impl AbstractMetric for Score {
    fn can_parallelize(&self) -> bool {
        true
    }

    fn get_name(&self) -> String {
        String::from("score")
    }

    async fn get_metric(&mut self, message: Message, _sequence_no: u32) -> MetricUpdate {
        match message {
            Message::Adventures(rank) => MetricUpdate {
                metric_name: self.get_name(),
                updates: HashMap::from([(rank.uid, rank.score as f32)]),
            },
            Message::Pxls(user) => user
                .discord_tag
                .map(|tag| MetricUpdate {
                    metric_name: self.get_name(),
                    updates: HashMap::from([(
                        "DISCORD-".to_string() + &user.pxls_username,
                        user.score as f32,
                    )]),
                })
                .unwrap_or(MetricUpdate::empty_with_name(self.get_name())),
            Message::IronmousePixels(user) => if let Some(IRONMOUSE_NEURO_FACTION) = user.faction {
                MetricUpdate {
                metric_name: self.get_name(),
                updates: self
                    .twitch
                    .get_user_from_username(user.pxls_username)
                    .await
                    .map(|info| HashMap::from([(info._id, user.score as f32)]))
                        .unwrap_or(HashMap::new()),
                }
            } else {
                MetricUpdate::empty_with_name(self.get_name())
            },
            _ => MetricUpdate::empty_with_name(self.get_name()),
        }
    }
}
