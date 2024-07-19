use crate::_types::clptypes::{BadgeInformation, UserChatPerformance};
use crate::_types::leaderboardtypes::{LeaderboardExportItem, LeaderboardInnerState};
use log::{debug, info};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

const K: f32 = 2.0;

pub trait AbstractLeaderboard {
    fn new() -> Self
    where
        Self: Sized;

    fn get_name(&self) -> String;

    fn __get_state(&mut self) -> &mut HashMap<String, LeaderboardInnerState>;

    fn calculate_score(&self, performance: &UserChatPerformance) -> Option<f32>;

    fn read_initial_state(&mut self) {
        info!("Loading {} leaderboard...", self.get_name());
        let path = format!("{}.json", self.get_name());
        if !std::path::Path::new(&path).exists() {
            info!("{} leaderboard doesn't already exist.", self.get_name());
            return;
        }

        let data = fs::read_to_string(&path).expect("Unable to read file");
        let items: Vec<Value> = serde_json::from_str(&data).expect("JSON was not well-formatted");
        self.__get_state().extend(items.into_iter().map(|item| {
            let export_item: LeaderboardExportItem = serde_json::from_value(item).unwrap();
            (
                export_item.id.clone(),
                LeaderboardInnerState {
                    id: export_item.id.clone(),
                    username: export_item.username,
                    avatar: export_item.avatar,
                    badges: export_item.badges,
                    previous_rank: Some(export_item.rank),
                    elo: export_item.elo,
                    score: 0.0,
                },
            )
        }));

        info!("{} leaderboard loading ok", self.get_name());
    }

    fn update_leaderboard(&mut self, performance: UserChatPerformance) {
        debug!(
            "Updating {} leaderboard with performance: {:?}",
            self.get_name(),
            performance
        );
        if let Some(score) = self.calculate_score(&performance) {
            debug!("Score for the above is {}", score);

            let id = performance.id.clone();
            let entry = self
                .__get_state()
                .entry(performance.id)
                .or_insert(LeaderboardInnerState {
                    id,
                    username: performance.username.clone(),
                    avatar: performance.avatar.clone(),
                    badges: None,
                    previous_rank: None,
                    elo: 1200.0,
                    score: 0.0,
                });

            let badges: Vec<BadgeInformation> = performance
                .metadata
                .get("badges")
                .map(|badge_list| badge_list.get_badge_list().unwrap().clone())
                .unwrap_or_default();

            entry.score = score;
            entry.badges = Some(badges);
        }
    }

    fn save(&mut self) {
        info!("Saving {} leaderboard...", self.get_name());
        self.__calculate_new_elo();
        let to_save: Vec<LeaderboardExportItem> = self
            .__get_state()
            .values()
            .map(|inner_state| LeaderboardExportItem {
                id: inner_state.id.clone(),
                rank: 0,
                elo: inner_state.elo,
                username: inner_state.username.clone(),
                delta: 0,
                avatar: inner_state.avatar.clone(),
                badges: inner_state.badges.clone(),
            })
            .collect();

        // Update rank and delta
        let mut sorted_to_save = to_save.clone();
        sorted_to_save.sort_by(|a, b| b.elo.partial_cmp(&a.elo).unwrap());
        assert!(sorted_to_save.len() > 1, "Nothing to save!");

        let updated_to_save: Vec<LeaderboardExportItem> = sorted_to_save
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let mut updated_item = item.clone();
                updated_item.rank = (i + 1) as u32;
                if let Some(state) = self.__get_state().get(&item.id) {
                    if let Some(previous_rank) = state.previous_rank {
                        updated_item.delta = previous_rank as i64 - updated_item.rank as i64;
                    }
                }
                updated_item
            })
            .collect();

        // Save to file
        let path = format!("{}.json", self.get_name());
        let data =
            serde_json::to_string(&updated_to_save).expect("Unable to serialize leaderboard data");
        fs::write(path, data).expect("Unable to write file");
        info!("{} leaderboard saved", self.get_name());
    }

    fn __calculate_new_elo(&mut self) {
        let all_scores: Vec<f32> = self
            .__get_state()
            .values()
            .map(|state| state.score)
            .collect();
        let sample_scores = self.percentiles(&all_scores, 0.0, 100.0, 0.1);
        // Build a vector of sample users, where the first element is the score and the second element is the elo
        // The elo is the elo of the user in state with the closest score
        let sample_users: Vec<(f32, f32)> = sample_scores
            .iter()
            .map(|score| {
                let (closest_user, _closest_distance) = self.__get_state().values().fold(
                    (None, f32::MAX),
                    |(closest_user, closest_distance), state| {
                        let distance = (state.score - score).abs();
                        if distance < closest_distance {
                            (Some(state), distance)
                        } else {
                            (closest_user, closest_distance)
                        }
                    },
                );
                (*score, closest_user.unwrap().elo)
            })
            .collect();
        drop(all_scores);
        drop(sample_scores);

        // Calculate the new elo for each user
        self.__get_state().values_mut().for_each(|state| {
            let diff: f32 = sample_users
                .iter()
                .map(|(sample_score, sample_elo)| {
                    let won = state.score > *sample_score;
                    let p = 1.0 / (1.0 + 10.0_f32.powf((sample_elo - state.elo) / 400.0));
                    K * (won as u8 as f32 - p)
                })
                .sum();
            state.elo += diff;
        });
    }

    fn percentiles(&self, scores: &[f32], start: f32, end: f32, step: f32) -> Vec<f32> {
        let mut sorted_scores = scores.to_vec();
        sorted_scores.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let step_count = ((end - start) / step) as usize + 1;
        let chunk_size = (sorted_scores.len() as f32 / step_count as f32).ceil() as usize;
        let chunks = sorted_scores.chunks(chunk_size);
        let percentiles: Vec<f32> = chunks.map(|chunk| chunk[chunk.len() / 2]).collect();
        percentiles
    }
}
