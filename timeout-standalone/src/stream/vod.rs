use crate::traits::{panic_if_no_streamlink, RemoteAudioSource};
use std::process::{Child, Command, Stdio};

pub struct TwitchVOD {
    process: Child,
}

impl RemoteAudioSource for TwitchVOD {
    fn get_compatible_stdout(&self) -> std::process::ChildStdout {
        self.process.stdout.expect("has an stdout")
    }
}

impl TwitchVOD {
    pub fn new(vod_id: &str) -> Self {
        panic_if_no_streamlink();

        let streamlink = Command::new("streamlink")
            .arg(format!("https://twitch.tv/videos/{vod_id}"))
            .arg("audio")
            .arg("-O")
            .stdout(Stdio::piped())
            .spawn()
            .expect("can launch streamlink");

        TwitchVOD {
            process: streamlink,
        }
    }
}
