use std::{collections::HashSet, sync::Arc};

use websocket_shared::{AuthorId, Elo, LeaderboardEloEntry, LeaderboardElos};

use super::websocket::{LeaderboardPerformances, PerformancePoints};

const STARTING_ELO: f32 = 1200.0;

// from https://github.com/vanorsigma/neuro-chat-elo/blob/dirty-bilibili/rust/elo/src/leaderboards/leaderboardtrait.rs#L15
// I don't really know what this value means in practice but w/e
const K: f32 = 2.0;

#[derive(Debug, Clone)]
struct WorkingEntry {
    author_id: AuthorId,
    elo: Elo,
    performance_points: PerformancePoints,
}

pub struct EloProcessor {
    base_leaderboard: Arc<LeaderboardElos>,
}

impl EloProcessor {
    pub fn new(base_leaderboard: Arc<LeaderboardElos>) -> Self {
        Self { base_leaderboard }
    }

    pub fn run(&self, performances: &LeaderboardPerformances) -> LeaderboardElos {
        let mut players = <Vec<LeaderboardEloEntry> as Clone>::clone(&self.base_leaderboard)
            .into_iter()
            .map(|LeaderboardEloEntry { author_id, elo }| WorkingEntry {
                author_id,
                elo,
                performance_points: PerformancePoints::new(0.0),
            })
            .collect::<Vec<_>>();

        let existing_users: HashSet<_> =
            HashSet::from_iter(players.iter().map(|entry| entry.author_id.to_owned()));

        for (author_id, performance_points) in performances {
            if !existing_users.contains(author_id) {
                players.push(WorkingEntry {
                    author_id: author_id.to_owned(),
                    elo: Elo::new(STARTING_ELO),
                    performance_points: performance_points.to_owned(),
                });
            }
        }

        drop(existing_users);

        // it might be a better idea to order by (elo, author_id) or something just to make sure that this remains consistent between runs
        // as that'd allow stuff to stay reproducible.
        // Probably not *super* important but worth noting
        players.sort_unstable_by(|l, r| l.elo.get().total_cmp(&r.elo.get()));

        // I was going to implement the elo algorithm from the dirty-bilibili branch, but I'm not smart enough to understand it
        // https://github.com/vanorsigma/neuro-chat-elo/blob/dirty-bilibili/rust/elo/src/leaderboards/leaderboardtrait.rs
        // Instead, I'm going to try the thing I was talking about :Clueless:
        // https://github.com/vanorsigma/neuro-chat-elo/pull/27#issuecomment-2322841347

        let mut results = Vec::new();

        // This could probably have a rayon par_iter thrown at it if needed
        for player in players.iter() {
            let mut opponents = players
                .iter()
                .filter(|opponent| opponent.author_id != player.author_id)
                .collect::<Vec<_>>();

            opponents.sort_by(|l, r| {
                (l.elo.get() - player.elo.get())
                    .abs()
                    .total_cmp(&(r.elo.get() - player.elo.get()).abs())
            });

            // TODO: decide on a better starting value maybe?
            let mut budget = 100.0;
            let mut kept = Vec::new();

            for opponent in opponents {
                kept.push(opponent);

                // This scoring system might need to be updated?
                // i.e. something like ((opp_elo - player_elo) / 100.0).powi(3);
                // which would favour smaller distances
                let cost = (opponent.elo.get() - player.elo.get()).abs();
                let cost = cost + 1.0;
                budget -= cost;

                if budget <= 0.0 {
                    break;
                }
            }

            let mut elo_change = 0.0;

            for opponent in kept {
                // This doesn't account for draws but I guess in practice they're very unlikely to occur
                let won = player.performance_points.get() > opponent.performance_points.get();
                let won_float = match won {
                    true => 1.0,
                    false => 0.0,
                };
                let p = 1.0 / (1.0 + 10.0_f32.powf(opponent.elo.get() - player.elo.get()) / 400.0);

                elo_change += K * (won_float - p)
            }

            results.push(LeaderboardEloEntry {
                author_id: player.author_id.to_owned(),
                elo: Elo::new(player.elo.get() + elo_change),
            })
        }

        results.sort_by(|l, r| l.elo.get().total_cmp(&r.elo.get()).reverse());

        LeaderboardElos::new(results)
    }
}
