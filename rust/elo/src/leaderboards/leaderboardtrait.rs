use crate::_types::{
    clptypes::UserChatPerformance,
    leaderboardtypes::{
        export_item_to_inner_state, BadgeInformation, LeaderboardExport, LeaderboardExportItem,
        LeaderboardInnerState,
    },
};
use itertools::Itertools;
use log::{debug, info, warn};
use prost::Message;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::{fs, fs::File};

const K: f32 = 2.0;
const STARTING_ELO: f32 = 80.0;

trait PartialWindowable<'a, T: 'a, F: Fn(&&T) -> R, R> {
    /// Constructs a partial window.
    /// Qualifier determines based on two consequitive items, prev and
    /// next, whether they should be considered the same (true) or different (false)
    /// item.
    fn partial_window(&'a self, size: usize, qualifier: F)
        -> impl Iterator<Item = (usize, Vec<T>)>;
}

struct LossyWindow<T> {
    slice: Vec<T>,
    window_size: isize,
    index: isize,
}

impl<T> LossyWindow<T> {
    fn new(slice: Vec<T>, window_size: usize) -> Self {
        LossyWindow {
            slice,
            window_size: window_size as isize,
            index: 0,
        }
    }
}

impl<T: Clone> Iterator for LossyWindow<T> {
    type Item = (isize, Vec<T>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index as usize >= self.slice.len() {
            return None;
        }

        let start = (self.index - self.window_size / 2).max(0) as usize;
        let end = (self.index + (self.window_size / 2)).min(self.slice.len() as isize) as usize;
        let window = self.slice[start..end].to_vec();
        self.index += 1;

        Some(((self.index - start as isize) - 1, window))
    }
}

impl<'a, T, F, R> PartialWindowable<'a, T, F, R> for Vec<T>
where
    T: 'a + Clone,
    F: Fn(&&T) -> R + Copy + 'a,
    R: std::cmp::PartialEq + 'a,
{
    fn partial_window(
        &'a self,
        size: usize,
        qualifier: F,
    ) -> impl Iterator<Item = (usize, Vec<T>)> {
        let result = self
            .iter()
            .chunk_by(qualifier)
            .into_iter()
            .map(|(_, view)| view.collect_vec())
            .collect::<Vec<_>>();

        LossyWindow::new(result, size).flat_map(|(idx, window)| {
            let flattened_index: usize = window
                .iter()
                .take(idx as usize)
                .map(|inner_window| inner_window.len())
                .sum();

            (0..window[idx as usize].len()).map(move |jdx| {
                (
                    flattened_index as usize + jdx,
                    window
                        .clone()
                        .into_iter()
                        .flatten()
                        .collect::<Vec<_>>()
                        .into_iter()
                        .map(|item| item.clone())
                        .collect::<Vec<_>>(),
                )
            })
        })
    }
}

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
                        avatar: performance.avatar.clone(),
                        badges: None,
                        previous_rank: None,
                        elo: STARTING_ELO,
                        score: 0.0,
                    });

            let badges: Vec<BadgeInformation> = performance
                .metadata
                .get("badges")
                .map(|badge_list| badge_list.get_badge_list().unwrap().clone())
                .unwrap_or_default();

            entry.score = score;
            entry.avatar = performance.avatar;
            entry.badges = Some(badges);
        }
    }

    fn save_to_disk(&mut self) {
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

    fn save(&mut self) {
        info!("Saving {} leaderboard...", self.get_name());
        self.__calculate_new_elo();
        // NOTE: it's possible for people to shoot up in rankings.
        // This acts as a "second chance" for people who got overtaken to battle
        self.__calculate_new_elo();
        self.save_to_disk();
    }

    fn __calculate_new_elo(&mut self) {
        // Sort all users by old elo, create an immutable sliding window
        // on each user. Probably do chunking with references to achieve this effect
        let mut sorted_by_elo = self.__get_state().values_mut().collect::<Vec<_>>();

        sorted_by_elo.sort_unstable_by(|a, b| a.elo.partial_cmp(&b.elo).unwrap());

        // Based on the sliding window, do battle with the current scores.
        let new_states = sorted_by_elo
            .iter()
            .map(|ele| ele as &LeaderboardInnerState)
            .collect_vec()
            .partial_window(9, |item| item.elo)
            .map(|(relative_center, window)| {
                let mut current_state = window[relative_center].clone();
                let diff: f32 = window
                    .iter()
                    .enumerate()
                    .filter(|(idx, _)| *idx != relative_center)
                    .map(|(_, state)| {
                        let won = current_state.score > state.score;
                        let p =
                            1.0 / (1.0 + 10.0_f32.powf((state.elo - current_state.elo) / 400.0));
                        K * (won as u8 as f32 - p)
                    })
                    .sum();
                current_state.elo += diff;
                current_state
            })
            .collect::<Vec<_>>();

        // TODO: use a proper setter
        sorted_by_elo
            .iter_mut()
            .enumerate()
            .for_each(|(idx, state)| state.elo = new_states[idx].elo);
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
