//! Produces an audio stream from either a Twitch livestream or a Twitch VOD
pub mod livestream;
pub mod traits;
pub mod vod;
pub mod local;

use std::io::{Read, Write};
use std::process::{Child, Command, Stdio};

use tokio::sync::mpsc;

use self::traits::RemoteAudioSource;

pub struct FFMPEGDecorator {
    process: Child,
}

impl RemoteAudioSource for FFMPEGDecorator {
    fn get_out_channel(self) -> mpsc::Receiver<u8> {
        let (sender, receiver) = mpsc::channel(10000);
        tokio::task::spawn_blocking(move || {
            self.process
                .stdout
                .expect("has an stdout")
                .bytes()
                .filter_map(|b| b.ok())
                .for_each(|b| {
                    let _ = sender.blocking_send(b);
                });
        });

        receiver
    }
}

impl FFMPEGDecorator {
    pub fn wrap_around<T: traits::RemoteAudioSource>(remote: T) -> Self {
        let mut child_receiver = remote.get_out_channel();

        log::info!("Creating FFMPEG proxy");

        let mut ffmpeg = Command::new("ffmpeg")
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
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("can make ffmpeg process");

        let mut stdin = ffmpeg.stdin.take().expect("can get ffmpeg stdin");

        tokio::task::spawn_blocking(move || {
            let mut buffer = Vec::with_capacity(16000 * 100);
            while let Some(byte) = child_receiver.blocking_recv() {
                buffer.push(byte);
                if buffer.len() >= 16000 {
                    let _ = stdin.write(buffer.as_slice());
                    buffer.clear();
                    let _ = stdin.flush();
                }
            }
        });

        FFMPEGDecorator { process: ffmpeg }
    }
}
