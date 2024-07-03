use tokio::sync::mpsc;

use crate::traits::{panic_if_no_streamlink, RemoteAudioSource};
use std::process::{Child, Command, Stdio};
use std::io::Read;

pub struct TwitchLiveStream {
    process: Child,
}

impl RemoteAudioSource for TwitchLiveStream {
    fn get_out_channel(self) -> mpsc::Receiver<u8> {
        let (sender, receiver) = mpsc::channel(10000);
        tokio::spawn(async move {
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

impl TwitchLiveStream {
    pub fn new(channel_name: &str, oauth_token: Option<&str>) -> Self {
        panic_if_no_streamlink();

        let mut binding = Command::new("streamlink");
        let streamlink_partial = binding
            .arg(format!("https://twitch.tv/{channel_name}"))
            .arg("audio_only")
            .arg("--twitch-disable-ads")
            .arg("--twitch-low-latency");

        if let Some(token) = oauth_token {
            streamlink_partial
                .arg("--twitch-api-header=Authorization=OAuth")
                .arg(token);
        }

        TwitchLiveStream {
            process: streamlink_partial
                .arg("-O")
                .stdout(Stdio::piped())
                .spawn()
                .expect("can launch streamlink"),
        }
    }
}
