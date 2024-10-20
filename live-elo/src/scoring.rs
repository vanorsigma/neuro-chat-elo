use std::sync::Arc;

use lbo::scoring::ScoringSystem;

pub struct MessageCountScoring {}

impl MessageCountScoring {
    pub fn new() -> Self {
        Self {}
    }
}

impl ScoringSystem for MessageCountScoring {
    type Message = Arc<super::sources::Message>;
    type Performance = super::exporter::websocket::PerformancePoints;
    type Closed = ();

    fn score_message(&self, message: Self::Message) -> Self::Performance {
        match message.as_ref() {
            crate::sources::Message::Twitch(_) => {
                super::exporter::websocket::PerformancePoints::new(1.0)
            }
            crate::sources::Message::Discord(_) => {
                super::exporter::websocket::PerformancePoints::new(1.0)
            }
            crate::sources::Message::B2(_) => {
                super::exporter::websocket::PerformancePoints::new(1.0)
            }
        }
    }

    async fn close(self) -> Self::Closed {}
}
