use std::sync::Arc;

use lbo::performances::PerformanceProcessor;
use tokio::{sync::broadcast, task::JoinSet};
use tokio_util::sync::CancellationToken;
use tracing::warn;

pub struct FanoutPerformancesBuilder<M> {
    send: broadcast::Sender<Arc<M>>,
    recv: broadcast::Receiver<Arc<M>>,
    joinset: JoinSet<()>,
    cancellation_token: CancellationToken,
}

impl<M> FanoutPerformancesBuilder<M>
where
    M: Send + Sync + 'static,
{
    pub fn new() -> Self {
        let (send, recv) = broadcast::channel(10_000);

        Self {
            send,
            recv,
            joinset: JoinSet::new(),
            cancellation_token: CancellationToken::new(),
        }
    }

    pub fn add_performance_processor<P, C>(mut self, processor: P) -> Self
    where
        C: Send + 'static,
        P: PerformanceProcessor<Message = Arc<M>, Closed = C> + Send + 'static,
    {
        self.joinset.spawn(fanout_task(
            self.recv.resubscribe(),
            processor,
            self.cancellation_token.clone(),
        ));
        self
    }

    pub fn build(self) -> FanoutPerformances<M> {
        FanoutPerformances {
            send: self.send,
            joinset: self.joinset,
            cancellation_token: self.cancellation_token,
        }
    }
}

pub struct FanoutPerformances<M> {
    send: broadcast::Sender<Arc<M>>,
    joinset: JoinSet<()>,
    cancellation_token: CancellationToken,
}

impl<M> FanoutPerformances<M>
where
    M: Send + Sync + 'static,
{
    pub fn builder() -> FanoutPerformancesBuilder<M> {
        FanoutPerformancesBuilder::new()
    }
}

impl<M> PerformanceProcessor for FanoutPerformances<M>
where
    // Debug is required for broadcast::Error
    M: Send + Sync + std::fmt::Debug,
{
    type Message = M;
    type Closed = ();

    async fn process_message(&mut self, message: Self::Message) {
        self.send.send(Arc::new(message)).unwrap();
    }

    async fn close(mut self) -> Self::Closed {
        self.cancellation_token.cancel();

        while let Some(result) = self.joinset.join_next().await {
            match result {
                Ok(_) => (),
                Err(error) => warn!(?error, "error cancelling fanout subtask"),
            }
        }
    }
}

async fn fanout_task<M, P, C>(
    mut recv: broadcast::Receiver<Arc<M>>,
    mut processor: P,
    cancellation_token: CancellationToken,
) where
    M: Send + Sync,
    C: Send,
    P: PerformanceProcessor<Message = Arc<M>, Closed = C> + Send,
{
    loop {
        let message = tokio::select! {
            message = recv.recv() => message,
            _ = cancellation_token.cancelled() => break,
        };

        let message = message.unwrap();
        processor.process_message(message).await;
    }
}
