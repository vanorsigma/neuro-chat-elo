use std::collections::VecDeque;
use std::io::Read;
use std::time::SystemTime;

use tokio::sync::broadcast;
use tokio::{join, select};
use tokio_stream::{Stream, StreamExt, StreamMap};
use twitch_irc::message::ServerMessage;

///! The lord of the house. Based on live audio and chat, it figures
///! out what to do. Handles the threading stuff too.
///! Because of the chat module, this MUST run in an async context
///! Aggregator will emit once every matching timeout
use crate::chat::Chat;
use crate::detection::TimeoutWordDetector;
use crate::stream::traits;
use crate::{ChatReceiver, TimeoutWordDetectorReceiver};

// We claim that timeouts can happen in a window of 10 seconds
const ALLOWED_WINDOW: f32 = 10.0;

#[derive(Debug, Clone)]
struct ChatTimeoutInformation {
    username: String,
    relative_timestamp: f32,
}

#[derive(Debug)]
pub struct ConfirmedTimeoutInformation {
    username: String,
    relative_timestamp: f32,
}

enum PossibleAggregateItems {
    Chat(ChatTimeoutInformation),
    DetectorTimestamp(f32),
}

struct AggregateIterator {
    chat: ChatReceiver,
    detector: TimeoutWordDetectorReceiver,
    start_timestamp: f32,
}

impl AggregateIterator {
    async fn only_take_deletions(
        chat: &mut ChatReceiver,
        start_timestamp: f32,
    ) -> ChatTimeoutInformation {
        while let Some(msg) = chat.next().await {
            match msg {
                ServerMessage::ClearMsg(m) => {
                    return ChatTimeoutInformation {
                        username: m.sender_login,
                        relative_timestamp: start_timestamp - m.server_timestamp.timestamp() as f32,
                    }
                }
                _ => continue,
            }
        }
        unreachable!("it's actually reachable but i'm lazy");
    }

    async fn take_timeout(detector: &mut TimeoutWordDetectorReceiver) -> f32 {
        while let Some(ts) = detector.next().await {
            return ts;
        }
        unreachable!("is actually reachabnle");
    }

    async fn get_next(&mut self) -> PossibleAggregateItems {
        let chat = &mut self.chat;
        let detector = &mut self.detector;
        select! {
            server_message = AggregateIterator::only_take_deletions(chat, self.start_timestamp) => {
                PossibleAggregateItems::Chat(server_message)
            }

            timeout_audio_ts = AggregateIterator::take_timeout(detector) => {
                PossibleAggregateItems::DetectorTimestamp(timeout_audio_ts)
            }
        }
    }
}

struct Aggregator {
    detector_queue: VecDeque<f32>,
    chat_queue: VecDeque<ChatTimeoutInformation>,
}

impl Aggregator {
    pub fn new<R: traits::RemoteAudioSource>(remote: R, chat: Chat, detector: TimeoutWordDetector) {
    }

    /// A blocking function that performs the aggregation
    pub async fn perform_aggregation<R: traits::RemoteAudioSource>(
        remote: R,
        chat: Chat,
        mut detector: TimeoutWordDetector,
        mut sender: broadcast::Sender<Vec<ConfirmedTimeoutInformation>>,
    ) {
        let mut detector_queue: VecDeque<f32> = VecDeque::new();
        let mut chat_queue: VecDeque<ChatTimeoutInformation> = VecDeque::new();

        let stdout = remote.get_compatible_stdout();
        let mut aggregate_iter = AggregateIterator {
            chat: chat.get_receiver().await,
            detector: detector.get_receiver().await,
            start_timestamp: SystemTime::now()
                .elapsed()
                .expect("can get system time")
                .as_secs_f32(),
        };

        let join_handle_audio = tokio::spawn(async move {
            // this particular task only focuses on piping output to the detector
            stdout
                .bytes()
                .filter_map(|b| b.ok())
                .for_each(|b| detector.ingest_byte(b));
        });

        // TODO join the above

        loop {
            match aggregate_iter.get_next().await {
                PossibleAggregateItems::Chat(chat_info) => {
                    println!("Chat information: {:#?}", chat_info);
                    chat_queue.push_back(chat_info);
                }
                PossibleAggregateItems::DetectorTimestamp(ts) => {
                    println!("Timestamp: {:#?}", ts);
                    detector_queue.push_back(ts);
                }
            }
            Self::confirm_timeouts_maybe(&mut chat_queue, &mut detector_queue, &mut sender);
        }

        // join!(join_handle, async move {
        //     loop {
        //         let relative_timestamp = audio_stream_detector.recv().await;
        //         detector.ingest_byte(123);
        //     }
        // });
    }

    fn confirm_timeouts_maybe(
        chat_queue: &mut VecDeque<ChatTimeoutInformation>,
        detector_queue: &mut VecDeque<f32>,
        sender: &mut broadcast::Sender<Vec<ConfirmedTimeoutInformation>>,
    ) {
        // invalidate any super old entries
        Self::invalidate_old_entries(chat_queue, detector_queue);

        // clone the detector queue. unlike chat, which is consumed
        // per timeout, detections create an AOE of timeout ranges
        let mut cloned_detector_queue = detector_queue.clone();
        let mut result: Vec<ConfirmedTimeoutInformation> = Vec::new();

        loop {
            // have to use a loop because i'll be mutating the queue as I go
            let oldest_chat = match chat_queue.front() {
                Some(c) => c,
                None => break,
            };

            let oldest_detected = match cloned_detector_queue.front() {
                Some(d) => d,
                None => break,
            };

            let comparison = oldest_chat.relative_timestamp - oldest_detected;
            if -ALLOWED_WINDOW < comparison && comparison < ALLOWED_WINDOW {
                result.push(ConfirmedTimeoutInformation {
                    username: oldest_chat.username.to_string(),
                    relative_timestamp: oldest_chat.relative_timestamp,
                });
                chat_queue.pop_front();
            } else if comparison > ALLOWED_WINDOW {
                cloned_detector_queue.pop_front();
            }
        }

        sender.send(result);
    }

    fn invalidate_old_entries(
        chat_queue: &mut VecDeque<ChatTimeoutInformation>,
        detector_queue: &mut VecDeque<f32>,
    ) {
        loop {
            let oldest_chat = match chat_queue.front() {
                Some(c) => c,
                None => return,
            };

            let oldest_detected = match detector_queue.front() {
                Some(d) => d,
                None => return,
            };

            let comparison = oldest_chat.relative_timestamp - oldest_detected;

            if comparison > ALLOWED_WINDOW {
                detector_queue.pop_front();
            } else if comparison < -ALLOWED_WINDOW {
                chat_queue.pop_front();
            } else {
                break;
            }
        }
    }
}
