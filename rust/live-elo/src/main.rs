use live_elo::provider::ProviderSet;
use log::{debug, info};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let twitch = twitch_utils::TwitchAPIWrapper::new().await.unwrap();
    let message_processor = elo::MessageProcessor::new(&twitch).await;
    let mut provider_set = ProviderSet::new(twitch);

    let (done_s, mut done_r) = tokio::sync::mpsc::channel::<()>(1);

    tokio::task::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        debug!("ctrl c pressed");
        drop(done_s);
    });

    loop {
        let message = tokio::select! {
            msg = provider_set.next_message() => msg,
            _ = done_r.recv() => {
                debug!("got finish signal");
                break;
            },
        };

        if message.is_none() {
            info!("finished reading messages from all providers");
            break;
        }
        
        message_processor.process_message(message.unwrap()).await;
    }

    info!("finished getting messages");
    let extras = provider_set.finish().await;
    if !extras.is_empty() {
        debug!("found {} extra messages", extras.len());
    }

    for extra in extras {
        message_processor.process_message(extra).await;
    }

    let performances = message_processor.finish().await;

    println!("{performances:?}");
}
