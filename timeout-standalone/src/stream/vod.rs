use tokio::sync::mpsc;

use crate::traits::{panic_if_no_streamlink, RemoteAudioSource};
use std::process::{Child, Command, Stdio};
use std::io::Read;

pub struct TwitchVOD {
    process: Child,
}

impl RemoteAudioSource for TwitchVOD {
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

impl TwitchVOD {
    pub fn new(vod_id: &str) -> Self {
        panic_if_no_streamlink();

        log::info!("Creating VOD source for {vod_id}");
        let streamlink = Command::new("streamlink")
            .arg(format!("https://twitch.tv/videos/{vod_id}"))
            .arg("audio")
            .arg("-Q")
            .arg("-O")
            .stdout(Stdio::piped())
            .spawn()
            .expect("can launch streamlink");

        TwitchVOD {
            process: streamlink,
        }
    }
}
