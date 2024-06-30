///! The lord of the house. Based on live audio and chat, it figures
///! out what to do. Handles the threading stuff too.
///! Because of the chat module, this MUST run in an async context

use crate::chat::Chat;
use crate::detection::TimeoutWordDetector;
use crate::stream::traits;

struct Aggregator {}

impl Aggregator {
    pub fn new<R: traits::RemoteAudioSource>(remote: R, chat: Chat, detector: TimeoutWordDetector) {
    }
}
