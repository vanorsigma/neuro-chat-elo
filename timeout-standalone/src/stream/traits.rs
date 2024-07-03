use std::process::{ChildStdout, Command};

use tokio::sync::mpsc;

pub trait RemoteAudioSource : Send + Sync {
    fn get_out_channel(self) -> mpsc::Receiver<u8>;
}

pub(super) fn panic_if_no_streamlink() {
    match Command::new("streamlink").arg("--version").output() {
        Ok(_) => {
            println!("Found streamlink, continuing...")
        }
        Err(_) => {
            panic!(
                "Streamlink not found. Please install streamlink with pip install streamlink"
            )
        }
    }
}
