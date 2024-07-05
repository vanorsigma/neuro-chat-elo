use serde::{Deserialize, Serialize};
use tempfile::{Builder, NamedTempFile, TempPath};
use zip::ZipArchive;
use log::info;

use std::fs::{self, File};
use std::io::{self, Read};
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

use crate::_types::twitchtypes::ChatLog;

#[allow(dead_code)]
const RELEASES_URL: &str = "https://api.github.com/repos/lay295/TwitchDownloader/releases/latest";

#[derive(Serialize, Deserialize, Debug)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GithubRelease {
    assets: Vec<GithubAsset>,
}

#[allow(dead_code)]
pub struct TwitchChatDownloader {
    executable_path: TempPath,
    downloaded: bool,
}

impl TwitchChatDownloader {
    #[allow(dead_code)]
    pub fn new() -> Self {
        TwitchChatDownloader {
            executable_path: NamedTempFile::new().unwrap().into_temp_path(),
            downloaded: false,
        }
    }

    #[allow(dead_code)]
    async fn download_executable(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        /*
        Downloads the latest executable from the releases page
        */
        println!("Downloading executable");

        let client = reqwest::Client::new();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("user-agent","CUSTOM_NAME/1.0".parse().unwrap());

        info!("Fetching latest release from GitHub");
        
        let response = client.get(RELEASES_URL).headers(headers).send().await?;
        
        if response.status().is_success() {
            let release: GithubRelease = response.json().await?;
            let asset = release.assets.iter().find(|a| a.name.ends_with("Linux-x64.zip"))
                .ok_or("No suitable asset found")?;

            let download_response = reqwest::get(&asset.browser_download_url).await?;
            let mut temp_zip = NamedTempFile::new()?;
            io::copy(&mut download_response.bytes().await?.as_ref(), &mut temp_zip)?;
            
            let mut zip = ZipArchive::new(temp_zip.reopen()?)?;
            let mut file = zip.by_name("TwitchDownloaderCLI")?;
            
            let mut exe_file = fs::File::create(&self.executable_path)?;
            io::copy(&mut file, &mut exe_file)?;
            exe_file.sync_all()?;
            fs::set_permissions(&self.executable_path, fs::Permissions::from_mode(0o700))?;

            drop(temp_zip);
            self.downloaded = true;
        } else {
            return Err(Box::new(io::Error::new(io::ErrorKind::Other, "Failed to fetch release")));
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn download_chat(&mut self, vod_id: &str) -> Result<ChatLog, Box<dyn std::error::Error>> {
        if !self.downloaded {
            println!("No executable downloaded, downloading...");
            self.download_executable().await?;
        }

        let output_file = Builder::new()
            .suffix(".json")
            .tempfile()?
            .into_temp_path();
        let output_path: String = output_file.to_str().unwrap().to_string().clone();
        output_file.close()?;
        
        let status = Command::new(&self.executable_path)
            .args(["chatdownload", "-u", vod_id, "-o", &output_path])
            .status()?;
    
        if !status.success() {
            return Err(Box::new(io::Error::new(io::ErrorKind::Other, "Failed to download chat")));
        }

        let mut file = File::open(&output_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        info!("Parsing JSON");
        let chat_log: ChatLog = serde_json::from_str(&contents)?;
        Ok(chat_log)
    }
}