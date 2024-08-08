use std::{fs::File, io, path::Path};

use log::info;

use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;
use zip::ZipArchive;

pub const USER_AGENT: &str = concat!(
    "neuro-chat-elo/0.1 ",
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (https://vanorsigma.github.io/neuro-chat-elo)"
);

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct GithubAsset {
    pub name: String,
    pub browser_download_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct GithubRelease {
    pub assets: Vec<GithubAsset>,
}

/// Gets the blob_url from the release_url that matches the pattern
pub async fn get_blob_url(
    releases_url: &str,
    matches_pattern: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    info!("Attempting to obtain blob from {releases_url}");
    Ok(reqwest::ClientBuilder::new()
        .user_agent(USER_AGENT)
        .build()
        .expect("Failed to create HTTP Client")
        .get(releases_url)
        .send()
        .await?
        .error_for_status()?
        .json::<GithubRelease>()
        .await?
        .assets
        .iter()
        .find(|a| a.name.rmatches(matches_pattern).last().is_some())
        .ok_or("No suitable asset found")?
        .browser_download_url
        .clone())
}

/// Downloads a zip file, and return a ZipArchive opened into it.
/// Path is a directory.
pub async fn extract_zip_blob(
    url: &str,
    path: &Path,
) -> Result<ZipArchive<File>, Box<dyn std::error::Error>> {
    info!("Downloading blob from {url} into {:#?}", path.to_str());
    if !path.is_dir() {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "Failed to fetch release",
        )));
    }
    let mut temp_file = NamedTempFile::new_in(path)?;
    io::copy(
        &mut reqwest::get(url).await?.bytes().await?.as_ref(),
        &mut temp_file,
    )?;
    Ok(ZipArchive::new(temp_file.reopen()?)?)
}
