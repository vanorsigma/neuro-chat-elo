use crate::_types::{
    clptypes::UserChatPerformance,
    leaderboardtypes::{
        export_item_to_inner_state, BadgeInformation, LeaderboardExport, LeaderboardExportItem,
        LeaderboardInnerState,
    },
};
use log::{debug, info, warn};
use prost::Message;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::{fs, fs::File};

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
        let path = format!("{}.bin", self.get_name());
        if !std::path::Path::new(&path).exists() {
            info!("{} leaderboard doesn't already exist.", self.get_name());
            return;
        }

        let mut file = File::open(path).unwrap();

        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();

        let leaderboard = LeaderboardExport::decode(&*buf).unwrap();

        for item in leaderboard.items {
            self.__get_state()
                .insert(item.id.clone(), export_item_to_inner_state(item));
        }

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

            let entry =
                self.__get_state()
                    .entry(performance.id.clone())
                    .or_insert(LeaderboardInnerState {
                        id: performance.id,
                        username: performance.username,
                        avatar: performance.avatar,
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
                badges: inner_state.badges.clone().unwrap_or_default(),
            })
            .collect();

        // Update rank and delta
        let mut sorted_to_save = to_save.clone();
        sorted_to_save.sort_by(|a, b| b.elo.partial_cmp(&a.elo).unwrap());
        if sorted_to_save.is_empty() {
            warn!("Nothing to save for leaderboard {}", self.get_name())
        }
        if sorted_to_save.is_empty() {
            warn!("Nothing to save for leaderboard {}", self.get_name())
        }

        let updated_to_save: Vec<LeaderboardExportItem> = sorted_to_save
            .into_iter()
            .enumerate()
            .map(|(i, mut item)| {
                item.rank = (i + 1) as u32;
                if let Some(state) = self.__get_state().get(&item.id) {
                    if let Some(previous_rank) = state.previous_rank {
                        item.delta = previous_rank as i64 - item.rank as i64;
                    }
                }
                item
            })
            .collect();

        let msg = LeaderboardExport::from(updated_to_save);
        let path = format!("{}.bin", self.get_name());
        let buf = msg.encode_to_vec();

        fs::File::create(path).unwrap().write_all(&buf).unwrap();

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
                let closest_user = self
                    .__get_state()
                    .values()
                    .min_by(|a, b| {
                        (a.score - score)
                            .abs()
                            .partial_cmp(&(b.score - score).abs())
                            .unwrap()
                    })
                    .unwrap();
                (*score, closest_user.elo)
            })
            .collect();

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
        if scores.is_empty() {
            return Vec::new();
        }

        let mut sorted_scores = scores.to_vec();
        sorted_scores.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let step_count = ((end - start) / step) as usize + 1;
        let chunk_size = (sorted_scores.len() as f32 / step_count as f32).ceil() as usize;
        let chunks = sorted_scores.chunks(chunk_size);
        let percentiles: Vec<f32> = chunks.map(|chunk| chunk[chunk.len() / 2]).collect();
        percentiles
    }

    fn is_discord_message(&self, performance: &UserChatPerformance) -> bool {
        if let MetadataTypes::Bool(true) = performance
            .metadata
            .get("is_discord_chat")
            .unwrap_or(&MetadataTypes::Bool(false))
        {
            true
        } else {
            false
        }
    }
}

#[macro_export]
macro_rules! is_message_origin {
    ($performance:expr, $tag:pat) => {
        matches!(
            $performance.metadata.get("chat_origin").unwrap_or(
                &$crate::_types::clptypes::MetadataTypes::ChatOrigin(MessageTag::None)
            ),
            $crate::_types::clptypes::MetadataTypes::ChatOrigin($tag)
        )
    };
}
