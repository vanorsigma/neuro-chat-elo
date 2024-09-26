use std::{collections::HashMap, sync::Arc};

use arc_swap::ArcSwap;
use lbo::exporter::Exporter;
use tokio::sync::RwLock;
use websocket_shared::{AuthorId, LeaderboardElos, LeaderboardName};

use super::{elo_calculator::EloProcessor, websocket::PerformancePoints};

// TODO: i don't like the name of this struct
#[derive(Clone)]
pub struct SharedHandle {
    elo_processors: Arc<HashMap<LeaderboardName, EloProcessor>>,
    current_performances:
        Arc<RwLock<HashMap<LeaderboardName, HashMap<AuthorId, PerformancePoints>>>>,
    current_leaderboard_data: Arc<ArcSwap<Option<Arc<HashMap<LeaderboardName, LeaderboardElos>>>>>,
}

impl SharedHandle {
    pub fn new(starting_leaderboards: Arc<HashMap<LeaderboardName, Arc<LeaderboardElos>>>) -> Self {
        Self {
            elo_processors: Arc::new(HashMap::from_iter(
                starting_leaderboards
                    .iter()
                    .map(|(name, values)| (name.to_owned(), EloProcessor::new(values.to_owned()))),
            )),
            current_performances: Arc::default(),
            current_leaderboard_data: Arc::default(),
        }
    }

    pub async fn get_leaderboard(&self) -> Arc<HashMap<LeaderboardName, LeaderboardElos>> {
        let value = self.current_leaderboard_data.load();

        match value.as_ref() {
            Some(existing_value) => existing_value.to_owned(),
            None => {
                // calculate elos and store in self.current_leaderboard_data
                let mut leaderboards = HashMap::new();
                let performances = self.current_performances.read().await;

                for (name, performances) in performances.iter() {
                    let processor = self.elo_processors.get(name).unwrap();
                    let leaderboard = processor.run(performances);
                    leaderboards.insert(name.to_owned(), leaderboard);
                }

                let leaderboards = Arc::new(leaderboards);
                self.current_leaderboard_data
                    .store(Arc::new(Some(leaderboards.clone())));

                leaderboards
            }
        }
    }

    pub async fn push_performance_change(
        &self,
        leaderboard: LeaderboardName,
        user: AuthorId,
        change: PerformancePoints,
    ) {
        let mut write = self.current_performances.write().await;
        let pp = write
            .entry(leaderboard)
            .or_default()
            .entry(user)
            .or_insert(PerformancePoints::zero());

        *pp = PerformancePoints::new(pp.get() + change.get());

        self.current_leaderboard_data.store(Arc::new(None));
    }

    // this function probably shouldn't be a near 1:1 copy of push_performance_change
    // maybe just make push_performance_change call this with a single element
    pub async fn push_performance_changes(
        &self,
        // FIXME: bruhge make a struct
        //        also use a &[...] or something idk sucks rn
        changes: Vec<(LeaderboardName, AuthorId, PerformancePoints)>,
    ) {
        let mut write = self.current_performances.write().await;

        for (leaderboard_name, author, change) in changes {
            let pp = write
                .entry(leaderboard_name)
                .or_default()
                .entry(author)
                .or_insert(PerformancePoints::zero());

            *pp = PerformancePoints::new(pp.get() + change.get());
        }

        self.current_leaderboard_data.store(Arc::new(None));
    }

    pub fn create_consumer_for_leaderboard(
        &self,
        leaderboard: LeaderboardName,
    ) -> SharedHandleConsumer {
        SharedHandleConsumer {
            leaderboard,
            handle: self.clone(),
        }
    }
}

pub struct SharedHandleConsumer {
    leaderboard: LeaderboardName,
    handle: SharedHandle,
}

impl Exporter for SharedHandleConsumer {
    type Performance = PerformancePoints;
    type AuthorId = AuthorId;
    type Closed = ();

    async fn export(&mut self, author_id: Self::AuthorId, performance: Self::Performance) {
        self.handle
            .push_performance_change(self.leaderboard.clone(), author_id, performance)
            .await;
    }

    async fn close(self) -> Self::Closed {}
}
