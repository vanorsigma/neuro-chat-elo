use bililive::{ConfigBuilder, Operation, Packet, Protocol, RetryConfig};
use futures::{SinkExt, StreamExt};
use lbo::sources::Source;
use serde_json::Value;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use super::Message;

#[derive(Debug)]
pub struct B2Message {
    pub user_id: String,
    pub message: String,
}

struct B2InternalMessage {
    uid: String,
    username: String,
    message: String,
    avatar: String,
}

pub struct B2MessageSourceHandle {
    mpsc_recv: mpsc::Receiver<B2Message>,
    task_join: tokio::task::JoinHandle<()>,
    cancellation_token: CancellationToken,
}

impl B2MessageSourceHandle {
    pub fn spawn(channel: u64, token: String) -> Self {
        let cancellation_token = CancellationToken::new();
        let (mpsc_send, mpsc_recv) = mpsc::channel(1000);
        let task_join = tokio::task::spawn(b2_source_inner(
            mpsc_send,
            channel,
            token,
            cancellation_token.clone(),
        ));

        Self {
            mpsc_recv,
            task_join,
            cancellation_token,
        }
    }
}

impl Source for B2MessageSourceHandle {
    type Message = Message;
    type Closed = ();

    async fn next_message(&mut self) -> Option<Self::Message> {
        self.mpsc_recv.recv().await.map(Message::B2)
    }

    async fn close(self) -> Self::Closed {
        self.cancellation_token.cancel();
        self.task_join.await.unwrap();
    }
}

impl From<B2InternalMessage> for B2Message {
    fn from(value: B2InternalMessage) -> Self {
        B2Message {
            user_id: value.uid,
            message: value.message,
        }
    }
}

async fn parse_b2_json(json: Value) -> Option<B2InternalMessage> {
    if let Some(cmd) = json.get("cmd") {
        if cmd == "DANMU_MSG" {
            if let Some(info) = json.get("info").and_then(|v| v.as_array()) {
                if let Some(user_info) = info.get(0).and_then(|v| v.as_array()) {
                    if let Some(user) = user_info.get(15).and_then(|v| v.get("user")) {
                        if let Some(base) = user.get("base") {
                            let face = base.get("face").and_then(|v| v.as_str()).unwrap_or("");
                            let name = base.get("name").and_then(|v| v.as_str()).unwrap_or("");
                            let (userhash, content) = user_info
                                .get(15)
                                .and_then(|v| v.get("extra"))
                                .and_then(|v| v.as_str())
                                .and_then(|v| serde_json::from_str::<Value>(v).ok())
                                .and_then(|v| {
                                    Some((v.get("user_hash")?.clone(), v.get("content")?.clone()))
                                })
                                .and_then(|(v1, v2)| {
                                    Some((v1.as_str()?.to_string(), v2.as_str()?.to_string()))
                                })
                                .unwrap_or((String::new(), String::new()));

                            return Some(B2InternalMessage {
                                uid: userhash,
                                username: name.to_string(),
                                message: content,
                                avatar: face.to_string(),
                            });
                        }
                    }
                }
            }
        }
    }

    return None;
}

async fn b2_source_inner(
    mpsc_send: mpsc::Sender<B2Message>,
    channel_uid: u64,
    token: String,
    cancellation_token: CancellationToken,
) {
    let config = ConfigBuilder::new()
        .sess_token(&token)
        .by_uid(channel_uid)
        .await
        .unwrap()
        .fetch_conf()
        .await
        .unwrap()
        .build();

    let mut stream = bililive::connect::tokio::connect_with_retry(config, RetryConfig::default())
        .await
        .unwrap();

    let jh = tokio::task::spawn(async move {
        loop {
            let packet = tokio::select! {
                p = stream.next() => p,
                _ = cancellation_token.cancelled() => break,
            };

            let packet = match packet {
                Some(p) => p,
                None => break,
            };

            match packet {
                Ok(packet) => {
                    if packet.op() == Operation::RoomEnterResponse {
                        info!("B2 is replying room enter response with heartbeat");
                        stream
                            .send(Packet::new(
                                Operation::HeartBeat,
                                Protocol::Json,
                                "{}".as_bytes(),
                            ))
                            .await
                            .expect("can send heartbeat");
                    }

                    if let Ok(json) = packet.json::<Value>() {
                        match parse_b2_json(json).await {
                            Some(val) => {
                                mpsc_send.send(val.into()).await.unwrap();
                            }
                            None => {
                                warn!("not a b2 message");
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("error in b2: {e:?}");
                }
            }
        }
    });

    jh.await.unwrap()
}
