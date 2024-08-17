use twitch_utils::TwitchAPIWrapper;

pub mod twitch;

pub struct ProviderSet {
    twitch_api: twitch_utils::TwitchAPIWrapper,
    collected_recv: tokio::sync::mpsc::UnboundedReceiver<elo::_types::clptypes::Message>,
    twitch_provider_task: tokio::task::JoinHandle<()>,
}

impl ProviderSet {
    pub fn new(twitch_api: TwitchAPIWrapper) -> Self {
        let (send, collected_recv) = tokio::sync::mpsc::unbounded_channel();

        let twitch_provider_task = tokio::task::spawn({
            let send = send.clone();
            let twitch_api = twitch_api.clone();
            async move {
                twitch::handle_messages(send, twitch_api.clone()).await;
            }
        });

        Self {
            twitch_api,
            collected_recv,
            twitch_provider_task,
        }
    }

    pub async fn next_message(&mut self) -> Option<elo::_types::clptypes::Message> {
        self.collected_recv.recv().await

        // None
    }

    pub async fn finish(mut self) -> Vec<elo::_types::clptypes::Message> {
        self.twitch_provider_task.abort();

        let mut extra_messages = Vec::new();

        while let Some(msg) = self.next_message().await {
            extra_messages.push(msg);
        }

        extra_messages
    }
}
