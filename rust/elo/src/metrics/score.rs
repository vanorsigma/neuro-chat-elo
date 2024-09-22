use std::collections::HashMap;
use std::sync::Arc;

use discord_utils::DiscordClient;
use twitch_utils::TwitchAPIWrapper;

use crate::_types::clptypes::{Message, MetricUpdate};
use crate::_types::{
    CASUAL_ID_SUFFIX, CASUAL_NEURO_FACTION, IRONMOUSE_ID_SUFFIX, IRONMOUSE_NEURO_FACTION,
};
use crate::metrics::metrictrait::AbstractMetric;

pub struct Score {
    twitch: Arc<TwitchAPIWrapper>,
    discord: Arc<DiscordClient>,
}

impl Score {
    pub fn new(twitch: Arc<TwitchAPIWrapper>, discord: Arc<DiscordClient>) -> Self {
        Self { twitch, discord }
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
            Message::Pxls(user) => {
                if let (Some(CASUAL_NEURO_FACTION), Some(discord_tag)) =
                    (user.faction, user.discord_tag.clone())
                {
                    MetricUpdate {
                        metric_name: self.get_name(),
                        updates: self
                            .discord
                            .get_username_author(discord_tag)
                            .await
                            .map(|info| {
                                HashMap::from([(info.id + CASUAL_ID_SUFFIX, user.score as f32)])
                            })
                            .unwrap_or(HashMap::new()),
                    }
                } else {
                    MetricUpdate::empty_with_name(self.get_name())
                }
            }
            Message::IronmousePixels(user) => {
                if let Some(IRONMOUSE_NEURO_FACTION) = user.faction {
                    MetricUpdate {
                        metric_name: self.get_name(),
                        updates: self
                            .twitch
                            .get_user_from_username(user.pxls_username)
                            .await
                            .map(|info| {
                                HashMap::from([(info._id + IRONMOUSE_ID_SUFFIX, user.score as f32)])
                            })
                            .unwrap_or(HashMap::new()),
                    }
                } else {
                    MetricUpdate::empty_with_name(self.get_name())
                }
            }
            _ => MetricUpdate::empty_with_name(self.get_name()),
        }
    }
}
