use anyhow::Result;
use hubbub::prelude::*;
use lbo::sources::Source;
use serde::Deserialize;
use serde::Serialize;
use tokio::pin;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use tracing::error;
use tracing::info;
use tracing::trace;

use super::Message;

#[derive(Deserialize, Clone, Debug)]
struct DiscordAuthor {
    id: String,
    avatar: Option<String>,
    global_name: Option<String>,
    username: String,
}

#[derive(Deserialize, Clone, Debug)]
struct DiscordMessageCreate {
    author: DiscordAuthor,
    guild_id: Option<String>,
    channel_id: Option<String>,
    content: String,
}

#[derive(Debug)]
pub struct DiscordMessage {
    pub message: String,
    pub author_id: String,
    pub channel_id: String,
}

impl From<DiscordMessageCreate> for DiscordMessage {
    fn from(value: DiscordMessageCreate) -> Self {
        DiscordMessage {
            message: value.content,
            author_id: value.author.id,
            channel_id: value.channel_id.unwrap_or("".to_string()),
        }
    }
}

pub struct DiscordMessageSourceHandle {
    mpsc_recv: mpsc::Receiver<DiscordMessage>,
    pub task_join: tokio::task::JoinHandle<()>,
    cancellation_token: CancellationToken,
}

pub struct DiscordHandleOptions {
    pub guild_id: String,
    pub channel_id: String,
    pub token: String,
}

impl DiscordMessageSourceHandle {
    pub fn spawn(options: DiscordHandleOptions) -> Self {
        let cancellation_token = CancellationToken::new();
        let (mpsc_send, mpsc_recv) = mpsc::channel(1000);

        let task_join = tokio::task::spawn(discord_source_inner(
            mpsc_send,
            options,
            cancellation_token.clone(),
        ));

        Self {
            mpsc_recv,
            task_join,
            cancellation_token,
        }
    }
}

impl Source for DiscordMessageSourceHandle {
    type Message = Message;
    type Closed = ();

    async fn next_message(&mut self) -> Option<Self::Message> {
        self.mpsc_recv.recv().await.map(Message::Discord)
    }

    async fn close(self) -> Self::Closed {
        self.cancellation_token.cancel();
        self.task_join.await.unwrap();
    }
}

/// To satisfy hubbub
struct DiscordApp {
    mpsc_send: mpsc::Sender<DiscordMessage>,
    channel_id: String,
    guild_id: String,
}

async fn discord_listener(
    _ctx: std::sync::Arc<tokio::sync::Mutex<hubbub::context::Context>>,
    _ws: Arc<tokio::sync::Mutex<Websocket>>,
    app: std::sync::Arc<tokio::sync::Mutex<DiscordApp>>,
    msg: hubbub::prelude::DiscordMessage,
) {
    if msg.event.as_ref().unwrap().as_str() == "READY" {
        info!("Discord Bot is listening for messages");
        return;
    }

    if msg.event.as_ref().unwrap().as_str() == "MESSAGE_CREATE" {
        trace!("Create message received {:?}", msg.data);
        let app = app.lock().await;
        let message_create = serde_json::from_value::<DiscordMessageCreate>(msg.data)
            .inspect(|msg| trace!("discord message: {:#?}", msg))
            .unwrap();

        if message_create.channel_id == Some(app.channel_id.clone())
            && message_create.guild_id == Some(app.guild_id.clone())
        {
            trace!("sending discord message because it matches channel & guild id");
            app.mpsc_send.send(message_create.into()).await.unwrap();
        }
    }

    trace!("events: {:?}", msg.event);
}

async fn discord_source_inner(
    mpsc_send: mpsc::Sender<DiscordMessage>,
    options: DiscordHandleOptions,
    cancellation_token: CancellationToken,
) {
    let mut client = Client::new(
        DiscordApp {
            mpsc_send,
            channel_id: options.channel_id,
            guild_id: options.guild_id,
        },
        Box::from(discord_listener),
    )
    .await
    .inspect_err(|e| error!("discord error: {:?}", e))
    .expect("can create discord client");

    client
        .token(options.token)
        .await
        .inspect_err(|e| error!("discord error: {:?}", e))
        .expect("discord token missing");

    client.login().await.expect("can't login with that token");
    tokio::select! {
        Err(e) = client.run() => {
            error!("Discord client error: {e:#?}")
        },
        _ = cancellation_token.cancelled() => return,
    }
}
