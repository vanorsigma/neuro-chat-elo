use tokio::sync::mpsc;

use crate::traits::RemoteAudioSource;
use std::process::{Child, Command, Stdio};
use std::io::Read;

pub struct Local {
    process: Child,
}

impl RemoteAudioSource for Local {
    fn get_out_channel(self) -> mpsc::Receiver<u8> {
        let (sender, receiver) = mpsc::channel(10000000);
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

impl Local {
    pub fn new(file_path: &str) -> Self {
        let ffmpeg = Command::new("ffmpeg")
            .arg("-i")
            .arg(file_path)
            .arg("-c:a")
            .arg("copy")
            .arg("-f")
            .arg("wav")
            .arg("-")
            .stdout(Stdio::piped())
            .spawn()
            .expect("can launch ffmpeg");

        Local {
            process: ffmpeg,
        }
    }
}
