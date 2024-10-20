use static_init::dynamic;
use twelf::{config, Layer};

#[config]
#[derive(Default)]
pub struct Config {
    #[serde(default = "default_rust_log")]
    pub rust_log: String,

    #[serde(default = "default_false")]
    pub twitch_enabled: bool,
    pub twitch_channel_name: Option<String>,

    #[serde(default = "default_false")]
    pub discord_enabled: bool,
    pub discord_livestream_channel_id: Option<String>,
    pub discord_livestream_guild_id: Option<String>,
    pub discord_token: Option<String>,

    #[serde(default = "default_false")]
    pub b2_enabled: bool,
    pub b2_livestream_channel: Option<u64>,
    pub b2_token: Option<String>,
}

fn default_false() -> bool {
    return false;
}

fn default_rust_log() -> String {
    return "info".into();
}

#[dynamic]
pub static GLOBAL_CONFIG: Config = {
    Config::with_layers(&[
        Layer::Yaml("config.yaml".into()),
        Layer::Env(Some("ELO_".into())),
    ])
    .unwrap_or(
        Config::with_layers(&[Layer::Env(Some("ELO_".into()))])
            .expect("should configure either config.yaml or env variables"),
    )
};
