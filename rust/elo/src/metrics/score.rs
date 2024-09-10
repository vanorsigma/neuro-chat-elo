use std::collections::HashMap;

use crate::_types::clptypes::{Message, MetricUpdate};
use crate::is_message_origin;
use crate::metrics::metrictrait::AbstractMetric;

#[derive(Default, Debug)]
pub struct Score;

impl Score {
    pub fn new() -> Self {
        Self {}
    }
}

impl AbstractMetric for Score {
    fn can_parallelize(&self) -> bool {
        true
    }

    fn get_name(&self) -> String {
        String::from("score")
    }

    fn get_metric(&mut self, message: Message, _sequence_no: u32) -> MetricUpdate {
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
            Message::IronmousePixels(user) => MetricUpdate {
                metric_name: self.get_name(),
                updates: HashMap::from([(
                    "TWITCH-".to_string() + &user.pxls_username,
                    user.score as f32,
                )]),
            },
            _ => MetricUpdate::empty_with_name(self.get_name()),
        }
    }
}
