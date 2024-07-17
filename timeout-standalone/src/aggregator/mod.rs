///! The lord of the house. Based on live audio and chat, it figures
///! out what to do. Handles the threading stuff too.
///! Because of the chat module, this MUST run in an async context
///! Aggregator will emit once every matching timeout
///!
///! Audio Dumping behaviour implicit assumptions:
///! - Chat timeouts are as fast, if not faster than the audio stream
///! - Timeout detection lags the audio stream, by not by much. This
///!   is ideally caught by the window
mod sliding_window;

use std::collections::VecDeque;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;
use tokio::select;
use tokio::sync::{broadcast, Mutex};
use tokio_stream::StreamExt;
use twitch_irc::message::{ClearChatMessage, ServerMessage};

use log::{debug, info, warn};

use crate::chat::Chat;
use crate::detection::TimeoutWordDetector;
use crate::stream::traits;
use crate::{ChatReceiver, TimeoutWordDetectorReceiver};

use self::sliding_window::SlidingWindow;

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

// username && sound_filename => true positive
// username only => potentially false negative
// sound_filename only => potentially false positive
// timestamp priority: sound first, then chat timestamp
#[derive(Serialize)]
struct TrainingDataOutput {
    sound_filename: String,
    detected: bool,
    username: Option<String>,
    relative_timestamp: f32,
}

#[derive(Clone)]
pub struct TrainingDataSettings {
    pub training_data_output_enabled: bool,
    pub duration_per_clip_in_seconds: f32,
    pub directory: String,
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
                    log::trace!(
                        "[CLEAR_CHAT] Received at {} with start_timestamp at {}",
                        time_now,
                        start_timestamp
                    );
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
    output_training_data: TrainingDataSettings,
    unconditional_purge_duration: u32,
) {
    let mut detector_queue: VecDeque<f32> = VecDeque::new();
    let mut chat_queue: VecDeque<ChatTimeoutInformation> = VecDeque::new();
    let mut ffmpeg_output = remote.get_out_channel();

    let window_size = output_training_data.duration_per_clip_in_seconds
        * crate::stream::consts::MIDDLEMAN_FFMPEG_SAMPLE_RATE as f32
        * crate::stream::consts::MIDDLEMAN_FFMPEG_CHANNELS as f32;
    let mut audio_sliding_window = Arc::new(Mutex::new(SlidingWindow::with_capacity(
        if output_training_data.training_data_output_enabled {
            window_size as usize / 2
        } else {
            0
        },
    ))); // try our best not to allocate any space if we can help it

    let mut aggregate_iter = AggregateIterator {
        chat: chat.get_receiver(),
        detector: detector.get_receiver(),
        start_timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("can get time since epoch")
            .as_secs_f32(),
    };

    // TODO: Evaluate if we need to use a thread at all
    let cloned_window = audio_sliding_window.clone();
    let join_handle_audio = tokio::spawn(async move {
        // this particular task only focuses on piping output to the detectorxs
        // TODO: if we perform some buffering here, we might be able to detect stuff faster.
        // it's not very urgent to do now, though
        // NOTE: feels like this will be really slow when output training data is enabled
        while let Some(byte) = ffmpeg_output.recv().await {
            detector.ingest_byte(byte);

            if output_training_data.training_data_output_enabled {
                cloned_window.lock().await.push(byte);
            }
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
        confirm_timeouts_maybe(
            &mut chat_queue,
            &mut detector_queue,
            &mut sender,
            unconditional_purge_duration,
            &output_training_data,
            audio_sliding_window.clone(),
        )
        .await;
    }

    // join!(join_handle, async move {
    //     loop {
    //         let relative_timestamp = audio_stream_detector.recv().await;
    //         detector.ingest_byte(123);
    //     }
    // });
}

async fn confirm_timeouts_maybe(
    chat_queue: &mut VecDeque<ChatTimeoutInformation>,
    detector_queue: &mut VecDeque<f32>,
    sender: &mut broadcast::Sender<Vec<ConfirmedTimeoutInformation>>,
    unconditional_purge_duration: u32,
    output_training_data: &TrainingDataSettings,
    audio_window: Arc<Mutex<SlidingWindow<u8>>>,
) {
    // invalidate any super old entries
    invalidate_old_entries(
        chat_queue,
        detector_queue,
        output_training_data,
        unconditional_purge_duration,
        audio_window.clone(),
    )
    .await;

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
            let res = chat_queue.pop_front();
            dump_audio_if_needed(
                output_training_data.clone(),
                &*audio_window.lock().await,
                Some(*oldest_detected),
                res,
            );
        } else if comparison > ALLOWED_WINDOW {
            cloned_detector_queue.pop_front();
        }
    }

    if result.len() > 0 {
        debug!("Emitting timeout events {:#?}", result);
        let _ = sender.send(result);
    }
}

async fn invalidate_old_entries(
    chat_queue: &mut VecDeque<ChatTimeoutInformation>,
    detector_queue: &mut VecDeque<f32>,
    output_training_data: &TrainingDataSettings,
    unconditional_purge_duration: u32,
    audio_window: Arc<Mutex<SlidingWindow<u8>>>,
) {
    debug!(
        "Now invalidating old queue items. Original sizes (chat, detector): {} {}",
        chat_queue.len(),
        detector_queue.len()
    );

    loop {
        let oldest_chat = chat_queue.front();
        let oldest_detected = detector_queue.front();
        let newest_chat = chat_queue.back();
        let newest_detected = detector_queue.back();

        debug!("Oldest Chat is: {:#?}", oldest_chat);
        debug!("Newest Chat is: {:#?}", newest_chat);
        debug!("Oldest Detected is: {:#?}", oldest_detected);
        debug!("Newest Detected is: {:#?}", newest_detected);

        let chat_detected_comparison = if let (Some(c), Some(d)) = (oldest_chat, oldest_detected) {
            Some(c.relative_timestamp - d)
        } else {
            None
        };

        let chat_span_comparison = if let (Some(new_c), Some(old_c)) = (newest_chat, oldest_chat) {
            Some(new_c.relative_timestamp - old_c.relative_timestamp)
        } else {
            None
        };

        let detected_span_comparison =
            if let (Some(new_d), Some(old_d)) = (newest_detected, oldest_detected) {
                Some(new_d - old_d)
            } else {
                None
            };

        if chat_detected_comparison
            .map(|cd| cd > ALLOWED_WINDOW)
            .unwrap_or(false)
            || detected_span_comparison
                .map(|d| d > unconditional_purge_duration as f32)
                .unwrap_or(false)
        {
            let res = detector_queue.pop_front();
            dump_audio_if_needed(
                output_training_data.clone(),
                &*audio_window.lock().await,
                res,
                None,
            )
        } else if chat_detected_comparison
            .map(|cd| cd < -ALLOWED_WINDOW)
            .unwrap_or(false)
            || chat_span_comparison
                .map(|c| c > unconditional_purge_duration as f32)
                .unwrap_or(false)
        {
            let res = chat_queue.pop_front();
            dump_audio_if_needed(
                output_training_data.clone(),
                &*audio_window.lock().await,
                None,
                res,
            )
        } else {
            break;
        }
    }
}

// Requires: either detector_timeout_info or chat_timeout_info is populated.
fn dump_audio_if_needed(
    output_training_data: TrainingDataSettings,
    audio_window: &SlidingWindow<u8>,
    detector_timeout_info: Option<f32>,
    chat_timeout_info: Option<ChatTimeoutInformation>,
) {
    if !output_training_data.training_data_output_enabled {
        return;
    }

    let take_size = output_training_data.duration_per_clip_in_seconds
        * crate::stream::consts::MIDDLEMAN_FFMPEG_SAMPLE_RATE as f32
        * crate::stream::consts::MIDDLEMAN_FFMPEG_CHANNELS as f32;
    let take_double_iterator = audio_window.take(take_size as usize);

    // NOTE: fire-and-forget, intentional spawn
    tokio::spawn(async move {
        let window: Vec<u8> = take_double_iterator.collect().await;

        debug!(
            "Dumping {}s of audio for training purposes; has chat: {}, has sound: {}",
            output_training_data.duration_per_clip_in_seconds,
            chat_timeout_info.is_some(),
            detector_timeout_info.is_some()
        );

        if let Some(ts) = detector_timeout_info {
            let filename = format!("{}{}", ts as u32, uuid::Uuid::new_v4().urn());
            _dump_audio(
                &window,
                &TrainingDataOutput {
                    sound_filename: format!("{}.wav", filename),
                    detected: true,
                    username: chat_timeout_info.map(|info| info.username),
                    relative_timestamp: ts,
                },
                &output_training_data,
                &filename,
            )
        } else if let Some(chat) = chat_timeout_info {
            let filename = format!(
                "{}{}",
                chat.relative_timestamp as u32,
                uuid::Uuid::new_v4().urn()
            );
            _dump_audio(
                &window,
                &TrainingDataOutput {
                    sound_filename: format!("{}.wav", filename),
                    detected: false,
                    username: Some(chat.username),
                    relative_timestamp: chat.relative_timestamp,
                },
                &output_training_data,
                &filename,
            )
        } else {
            warn!("dump_audio_if_needed doesn't seem to be used correctly");
        }
    });
}

fn _dump_audio(
    audio_window: &Vec<u8>,
    training_data_output: &TrainingDataOutput,
    training_data_settings: &TrainingDataSettings,
    filename: &str,
) {
    // create directory if does not exist
    std::fs::create_dir_all(&training_data_settings.directory).unwrap();

    let spec = hound::WavSpec {
        channels: crate::stream::consts::MIDDLEMAN_FFMPEG_CHANNELS as u16,
        sample_rate: crate::stream::consts::MIDDLEMAN_FFMPEG_SAMPLE_RATE,
        bits_per_sample: 32, // TODO: needs to be a constant, but we're basically assuming this everywhere
        sample_format: hound::SampleFormat::Float, // TODO: same for this as well
    };

    let mut writer = hound::WavWriter::create(
        Path::new(&training_data_settings.directory).join(format!("{}.wav", filename)),
        spec,
    )
    .unwrap();

    audio_window
        .chunks_exact(2)
        .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
        .for_each(|data| {
            let _ = writer.write_sample(data);
        });

    let mut json_file = File::create(
        Path::new(&training_data_settings.directory).join(format!("{}.json", filename)),
    )
    .unwrap();
    json_file
        .write_all(&serde_json::ser::to_vec(training_data_output).unwrap())
        .unwrap();
}
