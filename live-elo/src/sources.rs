pub mod twitch;
pub mod discord;

use discord::DiscordMessage;
use lbo::{message::AuthoredMesasge, sources::Source};
use tokio::{sync::mpsc, task::JoinSet};
use tokio_util::sync::CancellationToken;
use tracing::warn;
use twitch::TwitchMessage;
use websocket_shared::{AuthorId, DiscordId, TwitchId};

#[derive(Debug)]
pub enum Message {
    Twitch(TwitchMessage),
    Discord(DiscordMessage)
}

impl AuthoredMesasge for Message {
    type Id = AuthorId;

    fn author_id(&self) -> Self::Id {
        match self {
            Message::Twitch(message) => AuthorId::Twitch(TwitchId::new(message.author_id.clone())),
            Message::Discord(message) => AuthorId::Discord(DiscordId::new(message.author_id.clone())),
        }
    }
}

pub struct CancellableSource<S, M, C>
where
    S: Source<Message = M, Closed = C>,
{
    source: S,
    cancellation_token: CancellationToken,
}

impl<S, M, C> CancellableSource<S, M, C>
where
    S: Source<Message = M, Closed = C>,
{
    pub fn new(source: S, cancellation_token: CancellationToken) -> Self {
        Self {
            source,
            cancellation_token,
        }
    }
}

impl<S, M, C> Source for CancellableSource<S, M, C>
where
    S: Source<Message = M, Closed = C> + Send,
{
    type Message = M;
    type Closed = C;

    async fn next_message(&mut self) -> Option<Self::Message> {
        tokio::select! {
            message = self.source.next_message() => message,
            _ = self.cancellation_token.cancelled() => None,
        }
    }

    async fn close(self) -> Self::Closed {
        self.source.close().await
    }
}

pub struct TokioTaskSourceBuilder<M>
where
    M: Send,
{
    joinset: JoinSet<()>,
    mpsc_send: mpsc::Sender<M>,
    mpsc_recv: mpsc::Receiver<M>,
    cancel_token: CancellationToken,
}

impl<M> TokioTaskSourceBuilder<M>
where
    M: Send + 'static,
{
    pub fn new() -> Self {
        let (send, recv) = mpsc::channel(10_000);

        Self {
            joinset: tokio::task::JoinSet::new(),
            mpsc_send: send,
            mpsc_recv: recv,
            cancel_token: CancellationToken::new(),
        }
    }

    pub fn add_source<S, C>(mut self, source: S) -> Self
    where
        C: Send + 'static,
        S: Source<Message = M, Closed = C> + Send + 'static,
    {
        self.joinset.spawn(tokio_task_source_wrapper(
            source,
            self.mpsc_send.clone(),
            self.cancel_token.clone(),
        ));
        self
    }

    pub fn build(self) -> TokioTaskSource<M> {
        TokioTaskSource {
            joinset: self.joinset,
            mpsc_recv: self.mpsc_recv,
            cancel_token: self.cancel_token,
        }
    }
}

pub struct TokioTaskSource<M>
where
    M: Send,
{
    joinset: JoinSet<()>,
    mpsc_recv: mpsc::Receiver<M>,
    cancel_token: CancellationToken,
}

impl<M> TokioTaskSource<M>
where
    M: Send + 'static,
{
    pub fn builder() -> TokioTaskSourceBuilder<M> {
        TokioTaskSourceBuilder::new()
    }
}

impl<M> Source for TokioTaskSource<M>
where
    M: Send,
{
    type Message = M;
    type Closed = ();

    async fn next_message(&mut self) -> Option<Self::Message> {
        self.mpsc_recv.recv().await
    }

    async fn close(mut self) -> Self::Closed {
        self.cancel_token.cancel();

        while let Some(result) = self.joinset.join_next().await {
            match result {
                Ok(_) => (),
                Err(error) => warn!(?error, "error stopping background task"),
            }
        }
    }
}

async fn tokio_task_source_wrapper<S, M, C>(
    mut s: S,
    mpsc_send: mpsc::Sender<M>,
    cancellation_token: CancellationToken,
) where
    M: Send,
    C: Send,
    S: Source<Message = M, Closed = C> + Send,
{
    loop {
        let message = tokio::select! {
            message = s.next_message() => message,
            _ = cancellation_token.cancelled() => {
                s.close().await;
                break;
            },
        };

        match message {
            Some(message) => mpsc_send.send(message).await.unwrap(),
            None => break,
        }
    }
}
