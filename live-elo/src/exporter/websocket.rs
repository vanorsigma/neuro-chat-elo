use std::{collections::HashMap, sync::Arc, time::Duration};

use axum::{
    extract::{ws::WebSocket, State, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::Serialize;
use tokio::sync::{broadcast, mpsc, RwLock};
use tokio_util::sync::CancellationToken;
use tower_http::trace::TraceLayer;
use tracing::{debug, info, instrument, trace, warn};
use websocket_shared::{
    AuthorId, Elo, LeaderboardEloChanges, LeaderboardEloEntry, LeaderboardElos, LeaderboardName,
    LeaderboardPosistion, LeaderboardsChanges, OutgoingMessage,
};

use super::shared_processor::SharedHandle;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(transparent)]
pub struct PerformancePoints(f32);

// TODO: make this into a newtype smhsmh
pub type LeaderboardPerformances = HashMap<AuthorId, PerformancePoints>;

impl PerformancePoints {
    pub fn new(value: f32) -> Self {
        Self(value)
    }

    pub fn get(&self) -> f32 {
        self.0
    }

    pub const fn zero() -> Self {
        Self(0.0)
    }
}

impl From<LeaderboardStateEntry> for LeaderboardEloEntry {
    fn from(value: LeaderboardStateEntry) -> Self {
        Self {
            author_id: value.author_id,
            elo: value.elo,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LeaderboardStateEntry {
    pub author_id: AuthorId,
    pub elo: Elo,
    pub performance_points: PerformancePoints,
}

pub type LeaderboardStates = Vec<LeaderboardStateEntry>;

#[instrument(level = "trace", skip_all)]
pub async fn turn_batched_performances_into_real_leaderboards(
    shared_processor: SharedHandle,
    web_state: WebState,
    outgoing: mpsc::Sender<LeaderboardsChanges>,
    cancellation_token: CancellationToken,
) {
    let mut previous_leaderboard_states = shared_processor.get_leaderboard().await;
    let mut interval = tokio::time::interval(Duration::from_secs(5));

    loop {
        let new_leaderboard_states = shared_processor.get_leaderboard().await;
        let changes = find_changes(&previous_leaderboard_states, &new_leaderboard_states);

        outgoing.send(changes).await.unwrap();
        let mut leaderboard_states_write = web_state.current_leaderboards_states.write().await;
        *leaderboard_states_write = new_leaderboard_states.clone();
        drop(leaderboard_states_write);
        previous_leaderboard_states = new_leaderboard_states;

        tokio::select! {
            _ = interval.tick() => (),
            _ = cancellation_token.cancelled() => break,
        };
    }
}

type LeaderboardPerformancesDelta = HashMap<AuthorId, PerformancePoints>;
#[derive(Debug, Clone)]
struct IngestedPerformance {
    pub leaderboard_name: LeaderboardName,
    pub author_id: AuthorId,
    pub performance: PerformancePoints,
}

type FullBatchedPerformances = HashMap<LeaderboardName, LeaderboardPerformancesDelta>;

#[instrument(level = "trace", skip_all)]
pub async fn batch_performance_updates(
    mut incoming: mpsc::Receiver<IngestedPerformance>,
    outgoing: mpsc::Sender<FullBatchedPerformances>,
) {
    loop {
        let first = incoming.recv().await;
        trace!(?first, "got first message in batch");

        let first = match first {
            Some(first) => first,
            None => break,
        };

        let mut read = Vec::new();
        read.push(first);

        let _ = tokio::time::timeout(
            // FIXME: raise this probably
            Duration::from_secs(5),
            read_as_many_as_possible(&mut incoming, &mut read),
        )
        .await;

        let before_size = read.len();
        let batch = squash_batch(read);
        trace!(
            new_size = batch.len(),
            old_size = before_size,
            "squashed batch"
        );

        outgoing.send(batch).await.unwrap();
    }

    debug!("batch_performance_updates loop exited")
}

fn squash_batch(read: Vec<IngestedPerformance>) -> FullBatchedPerformances {
    let mut batch = FullBatchedPerformances::new();

    for IngestedPerformance {
        leaderboard_name,
        author_id,
        performance,
    } in read
    {
        let leaderboard = batch.entry(leaderboard_name).or_default();
        let author_score = leaderboard
            .entry(author_id)
            .or_insert(PerformancePoints::new(0.0));
        *author_score = PerformancePoints::new(author_score.get() + performance.get());
    }

    batch
}

async fn read_as_many_as_possible<T>(mpsc: &mut mpsc::Receiver<T>, into: &mut Vec<T>) {
    while let Some(value) = mpsc.recv().await {
        into.push(value);
    }
}

fn find_changes(
    from: &HashMap<LeaderboardName, LeaderboardElos>,
    to: &HashMap<LeaderboardName, LeaderboardElos>,
) -> LeaderboardsChanges {
    let mut changes = LeaderboardsChanges::new();
    for (name, before) in from {
        let leaderboard_changes = changes
            .get_mut()
            .entry(name.to_owned())
            .or_insert_with(LeaderboardEloChanges::new);
        let now = to.get(name).unwrap();

        for (index, now_at) in now.iter().enumerate() {
            if before.get(index).map(|b| b != now_at).unwrap_or(true) {
                leaderboard_changes.insert(LeaderboardPosistion::new(index), now_at.to_owned());
            }
        }
    }

    changes
}

pub struct WebServerHandle {
    server_task: tokio::task::JoinHandle<Result<(), std::io::Error>>,
    dependent_tasks: tokio::task::JoinSet<()>,
    // This needs to be here because otherwise the channel will close before any websockets connect
    _serialized_recv: broadcast::Receiver<Arc<SerializedOutgoingMessage>>,
    cancellation_token: CancellationToken,
}

impl WebServerHandle {
    pub async fn close(mut self) {
        self.cancellation_token.cancel();

        while let Some(result) = self.dependent_tasks.join_next().await {
            match result {
                Ok(_) => (),
                Err(error) => warn!(?error, "subtask failed when joining"),
            }
        }

        match self.server_task.await {
            Ok(_) => (),
            Err(error) => warn!(?error, "server task failed to join"),
        }
    }
}

async fn run_webserver(
    shared_processor: SharedHandle,
    cancellation_token: CancellationToken,
) -> WebServerHandle {
    let (serialized_send, serialized_recv) = broadcast::channel(10);
    let current_leaderboards_states =
        Arc::new(RwLock::new(shared_processor.get_leaderboard().await));

    let state = WebState {
        serialized_send: serialized_send.clone(),
        current_leaderboards_states: current_leaderboards_states.clone(),
        cancellation_token: cancellation_token.clone(),
    };

    let mut dependent_tasks = tokio::task::JoinSet::new();
    let (changes_send, changes_recv) = mpsc::channel(10_000);
    dependent_tasks.spawn(turn_batched_performances_into_real_leaderboards(
        shared_processor,
        state.clone(),
        changes_send,
        cancellation_token.clone(),
    ));
    dependent_tasks.spawn(serialize_changes(
        changes_recv,
        serialized_send,
        cancellation_token.clone(),
    ));

    let router = Router::new()
        .layer(TraceLayer::new_for_http())
        .route("/websocket", get(get_websocket))
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind("localhost:8000")
        .await
        .unwrap();

    let webserver_task = {
        let cancellation_token = cancellation_token.clone();
        tokio::task::spawn(async move {
            let local_addr = listener.local_addr().unwrap();
            info!(?local_addr, "starting listener");
            axum::serve(listener, router)
                .with_graceful_shutdown(cancel_token_wrapper(cancellation_token))
                .await
        })
    };

    WebServerHandle {
        server_task: webserver_task,
        dependent_tasks,
        _serialized_recv: serialized_recv,
        cancellation_token,
    }
}

async fn cancel_token_wrapper(cancellation_token: CancellationToken) {
    cancellation_token.cancelled().await;
}

async fn get_websocket(ws: WebSocketUpgrade, State(state): State<WebState>) -> impl IntoResponse {
    // TODO: make sure that these limits are only applied to incoming messages, and if so
    //       they could probably be lowered further
    ws.max_frame_size(2048)
        .max_message_size(2048)
        .on_upgrade(move |ws| handle_websocket(ws, state))
}

#[derive(Clone)]
struct WebState {
    serialized_send: broadcast::Sender<Arc<SerializedOutgoingMessage>>,
    current_leaderboards_states: Arc<RwLock<Arc<HashMap<LeaderboardName, LeaderboardElos>>>>,
    cancellation_token: CancellationToken,
}

async fn handle_websocket(mut ws: WebSocket, state: WebState) {
    let initial_state_lock = state.current_leaderboards_states.read().await;
    // This needs to happen whilst we have the read guard, as otherwise there's a risk
    // that we get a batch that has already been applied.
    // I don't think that would actually cause an issue come to think of it, but I'm unsure
    let mut batch_updater_recv = state.serialized_send.subscribe();
    let initial_state = initial_state_lock.clone();
    drop(initial_state_lock);
    // TODO: this should probably be stored somewhere to mitigate the effect on CPU usage if
    //       many clients connect at once
    let initial_state_serialized = serde_json::to_vec(&OutgoingMessage::InitialLeaderboards {
        leaderboards: initial_state.as_ref().to_owned(),
    })
    .unwrap();
    ws.send(axum::extract::ws::Message::Binary(initial_state_serialized))
        .await
        .unwrap();

    let cancellation_token = state.cancellation_token.clone();

    loop {
        let message = tokio::select! {
            message = ws.recv() => message.map(WebsocketMessageSide::FromWebsocket),
            message = batch_updater_recv.recv() => message.ok().map(WebsocketMessageSide::ToWebsocket),
            _ = cancellation_token.cancelled() => break,
        };

        if let None = message {
            break;
        }

        let message = message.unwrap();

        match message {
            WebsocketMessageSide::ToWebsocket(message) => {
                // It sucks that axum requires binary websockets to send messages using
                // `Vec<u8>`, so we have to clone out of the `Arc`
                // It might be worth adding a semaphore before `message.to_vec()` to ensure we don't end up with
                // hundreds of copies of the vec at the same time
                ws.send(axum::extract::ws::Message::Binary(message.to_vec()))
                    .await
                    .ok();
            }
            WebsocketMessageSide::FromWebsocket(message) => {
                match message {
                    Ok(message) => match message {
                        // Ideally there'd be some rate-limiting on pings, but surely nobody would do anything
                        // bad on the internet :Clueless:
                        axum::extract::ws::Message::Ping(_) => (),
                        // We're not expecting to get any messages from users, so just close the connection
                        // if they send anything
                        _ => break,
                    },
                    Err(error) => {
                        trace!(?error, "error reading message from websocket");
                        break;
                    }
                }
            }
        }
    }

    ws.close().await.ok();
}

type SerializedOutgoingMessage = Vec<u8>;

async fn serialize_changes(
    mut incoming: mpsc::Receiver<LeaderboardsChanges>,
    outgoing: broadcast::Sender<Arc<SerializedOutgoingMessage>>,
    cancellation_token: CancellationToken,
) {
    loop {
        let changes = tokio::select! {
            changes = incoming.recv() => changes,
            _ = cancellation_token.cancelled() => None,
        };

        let changes = match changes {
            Some(changes) => changes,
            None => break,
        };

        let serialized = serde_json::to_vec(&OutgoingMessage::Changes { changes }).unwrap();
        outgoing.send(Arc::new(serialized)).unwrap();
    }
}

enum WebsocketMessageSide {
    ToWebsocket(Arc<SerializedOutgoingMessage>),
    FromWebsocket(Result<axum::extract::ws::Message, axum::Error>),
}

pub struct UnstartedWebsocketServer {
    shared_processor: SharedHandle,
}

impl UnstartedWebsocketServer {
    pub fn new(shared_processor: SharedHandle) -> Self {
        Self { shared_processor }
    }

    pub async fn start(self) -> WebServerHandle {
        run_webserver(self.shared_processor, CancellationToken::new()).await
    }
}
