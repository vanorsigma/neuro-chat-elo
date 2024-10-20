use lbo::sources::Source;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use tracing::trace;

use super::Message;

#[derive(Debug)]
pub struct TwitchMessage {
    pub message: String,
    pub author_id: String,
}

pub struct TwitchMessageSourceHandle {
    mpsc_recv: mpsc::Receiver<TwitchMessage>,
    task_join: tokio::task::JoinHandle<()>,
    cancellation_token: CancellationToken,
}

impl TwitchMessageSourceHandle {
    pub fn spawn(channel: String) -> Self {
        let cancellation_token = CancellationToken::new();
        let (mpsc_send, mpsc_recv) = mpsc::channel(1000);
        let task_join = tokio::task::spawn(twitch_source_inner(
            mpsc_send,
            channel,
            cancellation_token.clone(),
        ));

        Self {
            mpsc_recv,
            task_join,
            cancellation_token,
        }
    }
}

impl Source for TwitchMessageSourceHandle {
    type Message = Message;
    type Closed = ();

    async fn next_message(&mut self) -> Option<Self::Message> {
        self.mpsc_recv.recv().await.map(Message::Twitch)
    }

    async fn close(self) -> Self::Closed {
        self.cancellation_token.cancel();
        self.task_join.await.unwrap();
    }
}

async fn twitch_source_inner(
    mpsc_send: mpsc::Sender<TwitchMessage>,
    channel: String,
    cancellation_token: CancellationToken,
) {
    let config = twitch_irc::ClientConfig::default();
    let (mut incoming_message, client) =
        twitch_irc::TwitchIRCClient::<twitch_irc::SecureTCPTransport, _>::new(config);

    let jh = tokio::task::spawn(async move {
        loop {
            let message = tokio::select! {
                message = incoming_message.recv() => message,
                _ = cancellation_token.cancelled() => break,
            };

            let message = match message {
                Some(message) => message,
                None => break,
            };

            if let twitch_irc::message::ServerMessage::Privmsg(message) = message {
                trace!(?message, "got irc message");
                mpsc_send
                    .send(TwitchMessage {
                        message: message.message_text,
                        // FIXME: this should be the users id, I just changed it to make it wayy more clear in the web thing
                        //        beacuse I'm too lazy to properly send this data over an api or something sane
                        author_id: message.sender.login,
                        // author_id: message.sender.id,
                    })
                    .await
                    .unwrap();
            }
        }
    });

    client.join(channel).unwrap();

    jh.await.unwrap()
}
