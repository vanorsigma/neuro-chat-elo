///! Package to handle chat. MUST run in an async context
use std::{future::Future, pin::Pin};

use std::task::{Context, Poll};

pub use rustpotter::SampleFormat;
use rustpotter::{Rustpotter, RustpotterConfig, ScoreMode};
use tokio::sync::broadcast::error::RecvError;
use tokio::{select, task::JoinHandle};
use tokio::{
    stream,
    sync::broadcast::{channel, Receiver, Sender},
};
use tokio_stream::Stream;
use tokio_util::sync::ReusableBoxFuture;
use twitch_irc::login::StaticLoginCredentials;
pub use twitch_irc::message::ServerMessage;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

use tokio::sync::oneshot;

pub struct ChatReceiver(
    ReusableBoxFuture<'static, (Result<ServerMessage, RecvError>, Receiver<ServerMessage>)>,
);

pub struct Chat {
    client: TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
    receiver: Receiver<ServerMessage>,
    chat_thread: JoinHandle<()>,
    cancel_receiver: oneshot::Receiver<()>, // TODO: might weant to do a notify here
}

impl Chat {
    pub fn new(channel_name: &str) -> Self {
        let config = ClientConfig::default();
        let (mut incoming_messages, client) =
            TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);
        let (sender, receiver) = channel(100);
        let (mut cancel_sender, cancel_receiver) = oneshot::channel();

        let join_handle = tokio::spawn(async move {
            loop {
                select! {
                    Some(message) = incoming_messages.recv() => {
                        let _ = sender.send(message);
                    }

                    // oneshot closes when dropped
                    _ = cancel_sender.closed() => {
                        break;
                    }
                }
            }
        });

        log::info!("Begin listening to Twitch Chat...");

        client.join(channel_name.to_string()).unwrap();

        Chat {
            client,
            receiver,
            chat_thread: join_handle,
            cancel_receiver,
        }
    }

    pub fn get_receiver(&self) -> ChatReceiver {
        ChatReceiver(ReusableBoxFuture::new(crate::make_future_from_rx(
            self.receiver.resubscribe(),
        )))
    }
}

impl Stream for ChatReceiver {
    type Item = ServerMessage;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let (result, rx) = futures::ready!(self.0.poll(cx));
        self.0.set(crate::make_future_from_rx(rx));

        match result {
            Ok(v) => std::task::Poll::Ready(Some(v)),
            Err(RecvError::Lagged(_)) => {
                cx.waker().wake_by_ref();
                std::task::Poll::Pending
            }
            Err(RecvError::Closed) => std::task::Poll::Ready(None),
        }
    }
}
