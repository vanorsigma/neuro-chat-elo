use static_init::dynamic;
use twelf::{config, Layer};

#[config]
#[derive(Default)]
pub struct Config {
    pub channel_name: String,
    #[serde(default = "default_rust_log")]
    pub rust_log: String,
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
