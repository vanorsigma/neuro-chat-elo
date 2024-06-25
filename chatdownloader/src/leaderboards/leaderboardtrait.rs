use std::collections::HashMap;
use std::fs;
use serde_json::Value;
use log::{info, debug};
use crate::_types::clptypes::{BadgeInformation, UserChatPerformance};
use crate::_types::leaderboardtypes::{LeaderboardExportItem, LeaderboardInnerState};

#[allow(dead_code)]
const K: f32 = 2.0;

#[allow(dead_code)]
pub trait AbstractLeaderboard {
    fn new() -> Self;

    fn get_name(&self) -> &str;

    fn __get_state(&mut self) -> &mut HashMap<String, LeaderboardInnerState>;

    fn calculate_score(&self, performance: &UserChatPerformance) -> Option<f32>;

    fn read_initial_state(&mut self) -> Result<(), std::io::Error> {
        info!("Loading {} leaderboard...", self.get_name());
        let path = format!("{}.json", self.get_name());
        if !std::path::Path::new(&path).exists() {
            info!("{} leaderboard doesn't already exist.", self.get_name());
            return Ok(());
        }

        let data = fs::read_to_string(&path).expect("Unable to read file");
        let items: Vec<Value> = serde_json::from_str(&data).expect("JSON was not well-formatted");
        self.__get_state().extend(items.into_iter().map(|item| {
            let export_item: LeaderboardExportItem = serde_json::from_value(item).unwrap();
            let id = export_item.id.clone();
            (id.clone(), LeaderboardInnerState {
                id,
                username: export_item.username,
                avatar: export_item.avatar,
                badges: export_item.badges,
                previous_rank: Some(export_item.rank),
                elo: export_item.elo,
                score: 0.0,
            })
        }));

        info!("{} leaderboard loading ok", self.get_name());

        Ok(())
    }

    fn update_leaderboard(&mut self, performance: UserChatPerformance) {
        debug!("Updating {} leaderboard with performance: {:?}", self.get_name(), performance);
        if let Some(score) = self.calculate_score(&performance) {
            debug!("Score for the above is {}", score);

            let id = performance.id.clone();
            let entry = self.__get_state().entry(performance.id).or_insert(LeaderboardInnerState {
                id,
                username: performance.username.clone(),
                avatar: performance.avatar.clone(),
                badges: None,
                previous_rank: None,
                elo: 1200.0,
                score: 0.0,
            });

            let badges: Vec<BadgeInformation> = performance.metadata["badges"]
                .as_array()
                .unwrap()
                .iter()
                .map(|badge| {
                    serde_json::from_value(badge.clone()).unwrap()
                }).collect();

            entry.score = score;
            entry.badges = Some(badges);
        }
    }

    fn save(&mut self) {
        info!("Saving {} leaderboard...", self.get_name());
        self.__calculate_new_elo();
        let to_save: Vec<LeaderboardExportItem> = self.__get_state().values().map(|inner_state| {
            LeaderboardExportItem {
                id: inner_state.id.clone(),
                rank: 0,
                elo: inner_state.elo,
                username: inner_state.username.clone(),
                delta: 0,
                avatar: inner_state.avatar.clone(),
                badges: inner_state.badges.clone(),
            }
        }).collect();

        // Update rank and delta
        let mut to_save = to_save;
        to_save.sort_by(|a, b| b.elo.partial_cmp(&a.elo).unwrap());
        assert!(to_save.len() > 1, "Nothing to save!");

        to_save[0].rank = 1;
        to_save[0].delta = match self.__get_state().get(&to_save[0].id) {
            Some(state) => {
                if let Some(previous_rank) = state.previous_rank {
                    if previous_rank > 0 {
                        previous_rank - 1
                    } else {
                        0
                    }
                } else {
                    0
                }
            },
            None => 0,
        };

        for (idx, item) in to_save.iter_mut().enumerate().skip(1) {
            let rank = to_save[idx - 1].rank + (item.elo < to_save[idx - 1].elo) as u16;
            item.delta = match self.__get_state().get(&item.id) {
                Some(state) => {
                    if let Some(previous_rank) = state.previous_rank {
                        if previous_rank > 0 {
                            previous_rank - rank
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                },
                None => 0,
            };
            item.rank = rank;
        }

        // Save to file
        let path = format!("{}.json", self.get_name());
        let data = serde_json::to_string(&to_save).expect("Unable to serialize leaderboard data");
        fs::write(&path, data).expect("Unable to write file");
        info!("{} leaderboard saved", self.get_name());
    }

    fn __calculate_new_elo(&mut self) {
        let all_scores: Vec<f32> = self.__get_state().values().map(|state| state.score).collect();
        let sample_scores = self.percentiles(&all_scores, 0.0, 100.0, 0.1);

        let sample_elos: Vec<f32> = sample_scores.iter().map(|&score| {
            self.__get_state().values()
                .map(|state| (state.elo, (state.score - score).abs()))
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .map(|(elo, _)| elo)
                .unwrap_or_default()
        }).collect();

        let mut score_differences: HashMap<String, f32> = self.__get_state().keys().map(|id| (id.clone(), 0.0)).collect();

        debug!("Sample Scores: {:?}", sample_scores);
        debug!("Sample Elos: {:?}", sample_elos);

        for state in self.__get_state().values() {
            for (idx, &sample_score) in sample_scores.iter().enumerate() {
                let won = (state.score > sample_score) as i32 as f32;
                let p = 1.0 / (1.0 + 10f32.powf((sample_elos[idx] - state.elo) / 400.0));
                *score_differences.get_mut(&state.id).unwrap() += K * (won - p);
            }
        }

        // Update all user's elo
        for (uid, &diff) in score_differences.iter() {
            if let Some(state) = self.__get_state().get_mut(uid) {
                state.elo += diff;
            }
        }
    }

    fn percentiles(&self, scores: &[f32], start: f32, end: f32, step: f32) -> Vec<f32> {
        let mut sorted_scores = scores.to_vec();
        sorted_scores.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut results = Vec::new();
        let step_count = ((end - start) / step) as usize + 1;
        for i in 0..step_count {
            let percentile = start + (i as f32) * step;
            let index = ((sorted_scores.len() - 1) as f32 * percentile / 100.0).round() as usize;
            results.push(sorted_scores[index]);
        }
        results
    }
}