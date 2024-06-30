use crate::traits::{panic_if_no_streamlink, RemoteAudioSource};
use std::process::{Child, Command, Stdio};

pub struct TwitchLiveStream {
    process: Child,
}

impl RemoteAudioSource for TwitchLiveStream {
    fn get_compatible_stdout(&self) -> std::process::ChildStdout {
        self.process.stdout.expect("has an stdout")
    }
}

impl TwitchLiveStream {
    pub fn new(channel_name: &str, oauth_token: Option<&str>) -> Self {
        panic_if_no_streamlink();

        let streamlink_partial = Command::new("streamlink")
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
