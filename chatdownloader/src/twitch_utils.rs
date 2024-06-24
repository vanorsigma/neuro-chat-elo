use reqwest;
use dotenv::dotenv;
use twitch_api::HelixClient;
use twitch_api::twitch_oauth2::{ClientId, ClientSecret, AppAccessToken};
use twitch_api::helix::videos::GetVideosRequest;

const USER_AGENT: &str = concat!(
    "neuro-chat-elo/0.1 ",
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (https://vanorsigma.github.io/neuro-chat-elo)"
);

#[allow(dead_code)]
pub struct Twitch {
    pub twitch: HelixClient<'static, reqwest::Client>,
    pub token: AppAccessToken
}

impl Twitch {
    #[allow(dead_code)]
    pub async fn new() -> Result<Self, reqwest::Error>{
        dotenv().ok();
        let client_id: ClientId = std::env::var("TWITCH_APPID")
            .map(ClientId::new)
            .expect("TWITCH_APPID must be set");

        let client_secret: ClientSecret = std::env::var("TWITCH_APPSECRET")
            .map(ClientSecret::new)
            .expect("TWITCH_APPSECRET must be set");

        let http_client = crate::twitch_utils::reqwest::ClientBuilder::new()
            .user_agent(USER_AGENT)
            .build()?;

        let twitch: HelixClient<'static, reqwest::Client> = twitch_api::HelixClient::with_client(http_client.clone());

        let token = AppAccessToken::get_app_access_token(
            &http_client,
            client_id,
            client_secret,
            vec![]
        ).await.unwrap();

        Ok(Self {
            twitch,
            token
        })
    }

    #[allow(dead_code)]
    pub async fn get_latest_vod_id(&self, ch_id: String) -> String {
        let request = GetVideosRequest::user_id(ch_id.clone());
        let response = self.twitch.req_get(request, &self.token);
        response.await.unwrap().data[0].id.clone().to_string()
    }
}