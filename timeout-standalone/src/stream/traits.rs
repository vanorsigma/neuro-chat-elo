use std::process::{ChildStdout, Command};

pub trait RemoteAudioSource {
    fn get_compatible_stdout(&self) -> ChildStdout;
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
