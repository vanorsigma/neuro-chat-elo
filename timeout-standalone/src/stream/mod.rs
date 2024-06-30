//! Produces an audio stream from either a Twitch livestream or a Twitch VOD
pub mod livestream;
pub mod traits;
pub mod vod;

use std::process::{Child, Command, Stdio};

use self::traits::RemoteAudioSource;

struct FFMPEGDecorator {
    process: Child,
}

impl RemoteAudioSource for FFMPEGDecorator {
    fn get_compatible_stdout(&self) -> std::process::ChildStdout {
        self.process.stdout.unwrap()
    }
}

impl FFMPEGDecorator {
    pub fn wrap_around<T: traits::RemoteAudioSource>(remote: T) -> Self {
        let child_stdout = remote.get_compatible_stdout();

        let ffmpeg = Command::new("ffmpeg")
            .arg("-hide_banner")
            .arg("-loglevel")
            .arg("error")
            .arg("-i")
            .arg("-")
            .arg("-ac")
            .arg("1")
            .arg("-ar")
            .arg("16000")
            .arg("-f")
            .arg("f32le")
            .arg("-")
            .stdin(child_stdout)
            .stdout(Stdio::piped())
            .spawn()
            .expect("can make ffmpeg process");

        FFMPEGDecorator { process: ffmpeg }
    }
}
