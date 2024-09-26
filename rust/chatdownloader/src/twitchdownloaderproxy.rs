use super::github::{extract_zip_blob, get_blob_url};
use log::info;
use tempfile::{Builder, NamedTempFile, TempDir, TempPath};

use std::fs::{self, File};
use std::io::{self, Read};
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

use twitch_utils::twitchtypes::ChatLog;

const RELEASES_URL: &str = "https://api.github.com/repos/lay295/TwitchDownloader/releases/latest";

pub struct TwitchChatDownloader {
    executable_path: TempPath,
    downloaded: bool,
}

impl TwitchChatDownloader {
    pub fn new() -> Self {
        TwitchChatDownloader {
            executable_path: NamedTempFile::new().unwrap().into_temp_path(),
            downloaded: false,
        }
    }

    async fn download_executable(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let temporary_directory = TempDir::new()?;
        let mut archive = extract_zip_blob(
            get_blob_url(RELEASES_URL, "Linux-x64.zip").await?.as_str(),
            temporary_directory.path(),
        )
        .await?;

        let mut file = archive.by_name("TwitchDownloaderCLI")?;

        let mut exe_file = fs::File::create(&self.executable_path)?;
        io::copy(&mut file, &mut exe_file)?;
        exe_file.sync_all()?;
        fs::set_permissions(&self.executable_path, fs::Permissions::from_mode(0o700))?;

        self.downloaded = true;
        Ok(())
    }

    pub async fn download_chat(
        &mut self,
        vod_id: &str,
    ) -> Result<ChatLog, Box<dyn std::error::Error>> {
        if !self.downloaded {
            println!("No executable downloaded, downloading...");
            self.download_executable().await?;
        }

        let output_file = Builder::new().suffix(".json").tempfile()?.into_temp_path();
        let output_path: String = output_file.to_str().unwrap().to_string().clone();
        output_file.close()?;
        // let output_path = "chat.json";

        let status = Command::new(&self.executable_path)
            .args(["chatdownload", "-u", vod_id, "-o", &output_path])
            .status()?;

        if !status.success() {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "Failed to download chat",
            )));
        }

        let mut file = File::open(&output_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        info!("Parsing JSON");
        let chat_log: ChatLog = serde_json::from_str(&contents)?;
        Ok(chat_log)
    }
}
