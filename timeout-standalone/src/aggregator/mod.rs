use std::collections::VecDeque;
use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};

use tokio::select;
use tokio::sync::broadcast;
use tokio_stream::StreamExt;
use twitch_irc::message::{ClearChatMessage, ServerMessage};

use log::{debug, info, warn};

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

#[derive(Debug, Clone)]
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

fn get_user_id_from_clear_chat_message(clear_chat_message: &ClearChatMessage) -> String {
    match clear_chat_message.action.to_owned() {
        twitch_irc::message::ClearChatAction::ChatCleared => String::new(),
        twitch_irc::message::ClearChatAction::UserBanned { user_login, .. } => user_login,
        twitch_irc::message::ClearChatAction::UserTimedOut { user_login, .. } => user_login,
    }
}

impl AggregateIterator {
    async fn only_take_deletions(
        chat: &mut ChatReceiver,
        start_timestamp: f32,
    ) -> ChatTimeoutInformation {
        while let Some(msg) = chat.next().await {
            match msg {
                ServerMessage::ClearChat(m) => {
                    let time_now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("can get system time")
                        .as_secs_f32();
                    log::trace!("[CLEAR_CHAT] Received at {} with start_timestamp at {}",
                                time_now, start_timestamp);
                    return ChatTimeoutInformation {
                        username: get_user_id_from_clear_chat_message(&m),
                        // NOTE: For some reason m.server_timestamp isn't accurate
                        // surely we get IRC messages on time so we just use system time
                        // relative_timestamp: m.server_timestamp.timestamp() as f32 - start_timestamp,
                        relative_timestamp: time_now - start_timestamp,
                    };
                }
                ServerMessage::Privmsg(m) => {
                    log::trace!("[CHAT] {}: {}", m.sender.login, m.message_text);
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
        unreachable!("is actually reachable");
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

/// A blocking function that performs the aggregation
pub async fn perform_aggregation<R: traits::RemoteAudioSource + 'static>(
    mut remote: R,
    chat: Chat,
    mut detector: TimeoutWordDetector,
    mut sender: broadcast::Sender<Vec<ConfirmedTimeoutInformation>>,
) {
    let mut detector_queue: VecDeque<f32> = VecDeque::new();
    let mut chat_queue: VecDeque<ChatTimeoutInformation> = VecDeque::new();
    let mut ffmpeg_output = remote.get_out_channel();

    let mut aggregate_iter = AggregateIterator {
        chat: chat.get_receiver(),
        detector: detector.get_receiver(),
        start_timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("can get time since epoch")
            .as_secs_f32(),
    };

    let join_handle_audio = tokio::spawn(async move {
        // this particular task only focuses on piping output to the detectorxs
        while let Some(byte) = ffmpeg_output.recv().await {
            detector.ingest_byte(byte);
        }
    });

    // TODO join the above, this is definitely not scuffed, no, not at all

    loop {
        match aggregate_iter.get_next().await {
            PossibleAggregateItems::Chat(chat_info) => {
                debug!("Chat Information received: {:#?}", chat_info);
                chat_queue.push_back(chat_info);
            }
            PossibleAggregateItems::DetectorTimestamp(ts) => {
                debug!("Timeout Wakeword detected at {:#?}", ts);
                detector_queue.push_back(ts);
            }
        }
        confirm_timeouts_maybe(&mut chat_queue, &mut detector_queue, &mut sender);
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
    invalidate_old_entries(chat_queue, detector_queue);

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
        if -ALLOWED_WINDOW < comparison && comparison <= ALLOWED_WINDOW {
            result.push(ConfirmedTimeoutInformation {
                username: oldest_chat.username.to_string(),
                relative_timestamp: oldest_chat.relative_timestamp,
            });
            chat_queue.pop_front();
        } else if comparison > ALLOWED_WINDOW {
            cloned_detector_queue.pop_front();
        }
    }

    if result.len() > 0 {
        debug!("Emitting timeout events {:#?}", result);
        let _ = sender.send(result);
    }
}

fn invalidate_old_entries(
    chat_queue: &mut VecDeque<ChatTimeoutInformation>,
    detector_queue: &mut VecDeque<f32>,
) {
    debug!(
        "Now invalidating old queue items. Original sizes (chat, detector): {} {}",
        chat_queue.len(),
        detector_queue.len()
    );

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

        if comparison >= ALLOWED_WINDOW {
            detector_queue.pop_front();
        } else if comparison < -ALLOWED_WINDOW {
            chat_queue.pop_front();
        } else {
            break;
        }
    }
}
