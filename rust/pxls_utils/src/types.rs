use serde::Deserialize;

/// TODO: This is currently a hack, because each structure has to be
/// unique for MessageTags to figure out the difference
#[derive(Deserialize, Debug, Clone)]
pub struct PxlsUser {
    #[serde(rename = "PXLS_USERNAME")]
    pub pxls_username: String,
    #[serde(rename = "DISCORD_TAG")]
    pub discord_tag: Option<String>,
    #[serde(rename = "FACTION")]
    pub faction: Option<u64>,
    #[serde(rename = "SCORE")]
    pub score: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct IronmousePxlsUser {
    #[serde(rename = "PXLS_USERNAME")]
    pub pxls_username: String,
    #[serde(rename = "DISCORD_TAG")]
    pub discord_tag: Option<String>,
    #[serde(rename = "FACTION")]
    pub faction: Option<u64>,
    #[serde(rename = "SCORE")]
    pub score: u64,
}

#[derive(Deserialize, Debug)]
pub(super) struct SqliteDump<U> {
    pub users: Vec<U>,
}
