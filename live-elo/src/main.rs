mod config;

use ::app_state::create_app_state;
use config::GLOBAL_CONFIG;
use hubbub::prelude::DiscordMessage;
use lbo::{performances::StandardLeaderboard, Pipeline};
use live_elo::{
    exporter::{shared_processor::SharedHandle, DummyExporter, MultiExporter},
    filter::DummyFilter,
    performances::FanoutPerformances,
    scoring::MessageCountScoring,
    sources::{
        bilibili::B2MessageSourceHandle,
        discord::{DiscordHandleOptions, DiscordMessageSourceHandle},
        twitch::TwitchMessageSourceHandle,
        CancellableSource, TokioTaskSource,
    },
};
use std::{str::FromStr, sync::Arc};
use tracing::{info, trace};
use websocket_shared::{LeaderboardElos, LeaderboardName};

#[tokio::main]
async fn main() {
    {
        use tracing_subscriber::prelude::*;

        tracing_subscriber::registry()
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(std::io::stderr)
                    .with_filter(
                        tracing_subscriber::EnvFilter::from_str(&GLOBAL_CONFIG.rust_log).unwrap(),
                    ),
            )
            .init();
    }

    let cancellation_token = tokio_util::sync::CancellationToken::new();

    let cancellation_signal_task = {
        let cancellation_token = cancellation_token.clone();

        tokio::task::spawn(async move {
            trace!("waiting for ctrl-c signal");
            tokio::signal::ctrl_c().await.ok();
            info!("info ctrl-c signal");
            cancellation_token.cancel();
        })
    };

    let shared_handle = SharedHandle::new(Arc::new(std::collections::HashMap::from([(
        LeaderboardName::new("message_count".to_string()),
        Arc::new(LeaderboardElos::new(Vec::new())),
    )])));

    let websocket_server =
        live_elo::exporter::websocket::UnstartedWebsocketServer::new(shared_handle.clone());

    let app_state = create_app_state();

    let mut tokio_task_builder = TokioTaskSource::builder();

    if GLOBAL_CONFIG.twitch_enabled {
        tokio_task_builder = tokio_task_builder.add_source(TwitchMessageSourceHandle::spawn(
            GLOBAL_CONFIG
                .twitch_channel_name
                .clone()
                .expect("need twitch channel name"),
        ));
    }

    if GLOBAL_CONFIG.discord_enabled {
        tokio_task_builder = tokio_task_builder.add_source(DiscordMessageSourceHandle::spawn(
            DiscordHandleOptions {
                channel_id: GLOBAL_CONFIG
                    .discord_livestream_channel_id
                    .clone()
                    .expect("should have livestream channel id"),
                guild_id: GLOBAL_CONFIG
                    .discord_livestream_guild_id
                    .clone()
                    .expect("should have livestream guild id"),
                token: GLOBAL_CONFIG
                    .discord_token
                    .clone()
                    .expect("should have discord token"),
            },
        ));
    }

    if GLOBAL_CONFIG.b2_enabled {
        tokio_task_builder = tokio_task_builder.add_source(B2MessageSourceHandle::spawn(
            GLOBAL_CONFIG
                .b2_livestream_channel
                .clone()
                .expect("should have bilibili channel id"),
            GLOBAL_CONFIG
                .b2_token
                .clone()
                .expect("shoudl ahve b2 token"),
        ));
    }

    let pipeline = Pipeline::builder()
        .source(CancellableSource::new(
            tokio_task_builder.build(),
            cancellation_token,
        ))
        .filter(DummyFilter::new())
        .performances(
            FanoutPerformances::builder()
                .add_performance_processor(StandardLeaderboard::new(
                    MessageCountScoring::new(),
                    MultiExporter::pair(
                        DummyExporter::new(),
                        shared_handle.create_consumer_for_leaderboard(LeaderboardName::new(
                            "message_count".to_string(),
                        )),
                    ),
                    app_state,
                ))
                .build(),
        )
        .build();

    let webserver_handle = websocket_server.start().await;
    let pipeline = pipeline.run().await.unwrap();
    tracing::debug!("pipeline finished");
    webserver_handle.close().await;
    pipeline.close().await;
    tracing::debug!("webserver handle finished");

    cancellation_signal_task.abort();
}
