///! Package to handle chat. MUST run in an async context
use tokio::{select, task::JoinHandle};
use twitch_irc::login::StaticLoginCredentials;
pub use twitch_irc::message::ServerMessage;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

use tokio::sync::broadcast::{channel, Receiver};
use tokio::sync::oneshot;

pub struct Chat {
    client: TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
    receiver: Receiver<ServerMessage>,
    chat_thread: JoinHandle<()>,
    cancel_receiver: oneshot::Receiver<()>,
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

        client.join(channel_name.to_string()).unwrap();

        Chat {
            client,
            receiver,
            chat_thread: join_handle,
            cancel_receiver,
        }
    }

    pub fn get_receiver(&self) -> Receiver<ServerMessage> {
        self.receiver.resubscribe()
    }
}
