use super::github::{extract_zip_blob, get_blob_url};
use chrono::{DateTime, Utc};
use discord_utils::DiscordChatLogs;
use log::info;
use tempfile::{Builder, TempDir};

use std::fs::{self, File};
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

const RELEASES_URL: &str =
    "https://api.github.com/repos/Tyrrrz/DiscordChatExporter/releases/latest";

pub struct DiscordChatDownloader {
    executable_directory: TempDir,
    executable_path: String,
    downloaded: bool,
}

/// Downloads Discord Chat Logs
impl DiscordChatDownloader {
    pub fn new() -> Self {
        let temporary_directory = TempDir::new().expect("Cannot create temporary directory");
        let mut executable_path_buf = temporary_directory.path().to_path_buf();
        executable_path_buf.push("DiscordChatExporter.Cli");

        DiscordChatDownloader {
            executable_directory: temporary_directory,
            executable_path: executable_path_buf
                .to_str()
                .expect("can construct executable")
                .to_string(),
            downloaded: false,
        }
    }

    /// Downloads the Discord Chat exporter executable from the releases patch
    async fn download_executable(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        extract_zip_blob(
            get_blob_url(RELEASES_URL, "DiscordChatExporter.Cli.linux-x64.zip")
                .await?
                .as_str(),
            self.executable_directory.path(),
        )
        .await?
        .extract(self.executable_directory.path())?;

        if !fs::exists(Path::new(self.executable_path.as_str()))? {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::NotFound,
                "Expected executable not found",
            )));
        };

        fs::set_permissions(&self.executable_path, fs::Permissions::from_mode(0o700))?;
        self.downloaded = true;
        Ok(())
    }

    pub async fn download_chat(
        &mut self,
        start_datetime: DateTime<Utc>,
        channel_id: &str,
        discord_token: &str,
    ) -> Result<DiscordChatLogs, Box<dyn std::error::Error>> {
        if !self.downloaded {
            info!("No executable downloaded, downloading...");
            self.download_executable().await?;
        }

        let output_path = Builder::new()
            .suffix(".json")
            .tempfile()?
            .into_temp_path()
            .keep()?
            .as_path()
            .to_str()
            .ok_or("cannot create temporary json")?
            .to_string();

        if !Command::new(&self.executable_path)
            .args([
                "export",
                "-c",
                channel_id,
                "-t",
                discord_token,
                "-f",
                "Json",
                "-o",
                output_path.as_str(),
                "--after",
                start_datetime
                    .format("%m/%d/%Y %I:%M %p")
                    .to_string()
                    .as_str(),
                "--utc",
            ])
            .status()?
            .success()
        {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "Failed to download Discord chat",
            )));
        }

        info!("Parsing JSON");
        Ok(serde_json::from_reader::<File, DiscordChatLogs>(
            File::open(&output_path)?,
        )?)
    }
}
