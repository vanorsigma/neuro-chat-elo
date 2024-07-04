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

impl TwitchLiveStream {
    pub fn new(channel_name: &str, oauth_token: Option<&str>) -> Self {
        panic_if_no_streamlink();

        let mut binding = Command::new("streamlink");
        let mut streamlink_partial = binding;
        if let Some(token) = oauth_token {
            streamlink_partial
                .arg(format!("--twitch-api-header=Authorization=OAuth {token}"));
        }

        log::info!("Creating livestream source for {channel_name}");
        TwitchLiveStream {
            process: streamlink_partial
                .arg(format!("https://twitch.tv/{channel_name}"))
                .arg("audio_only")
                .arg("--twitch-disable-ads")
                .arg("--twitch-low-latency")
                .arg("-Q")
                .arg("-O")
                .stdout(Stdio::piped())
                .spawn()
                .expect("can launch streamlink"),
        }
    }
}
