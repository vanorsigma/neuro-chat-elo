use log::debug;

use crate::_types::twitchtypes::Comment;
use crate::metrics::metrictrait::AbstractMetric;

use std::collections::HashMap;

const WEIGHT_COPYPASTA: f32 = 0.3;
const CHAIN_GRACE: u32 = 10;
const MATCHING_THRESHOLD: f32 = 0.6;

#[derive(Default, Debug)]
pub struct CopypastaLeader {
    history: Vec<(u32, String, String, u32)>,
}

impl AbstractMetric for CopypastaLeader {
    async fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }
    
    fn can_parallelize(&self) -> bool {
        false
    }

    fn get_name(&self) -> String {
        String::from("copypasta")
    }

    fn get_metric(&mut self, comment: Comment, sequence_no: u32) -> HashMap<String, f32> {
        let text = comment.message.fragments.iter()
            .map(|fragment| fragment.text.clone())
            .collect::<Vec<String>>()
            .join(" ");

        if text.is_empty() {
            return HashMap::new();
        }

        // Evaluate or initialize the list
        if self.history.is_empty() {
            self.history.push((sequence_no, text.clone(), comment.commenter._id.clone(), sequence_no));
        }

        debug!("Size of heap: {}", self.history.len());

        // Find the best matching string in the list 
        let mut best_match = None;
        let mut best_match_score = 0.0;
        for item in self.history.iter() {
            let lcs = Self::lcs(&text, &item.1);
            let score = lcs.len() as f32 / text.len().max(item.1.len()) as f32;
            if score > best_match_score {
                best_match = Some(item);
                best_match_score = score;
            }
        }

        // If the best match is above the threshold, update the list
        if best_match_score > MATCHING_THRESHOLD {
            let item = best_match.unwrap();
            self.history.push((sequence_no, text.clone(), comment.commenter._id.clone(), item.3));
        } else {
            self.history.push((sequence_no, text.clone(), comment.commenter._id.clone(), sequence_no));
        }

        // Sort the list
        self.history.sort_by_key(|item| item.0);
        debug!("Items on heap: {:?}", self.history.iter().map(|item| item.0).collect::<Vec<u32>>());

        debug!("First item on heap has seq no: {:?}", self.history.first().unwrap().0);
        debug!("Last item on heap has seq no: {:?}", self.history.last().unwrap().0);

        // Evict old list top
        let mut result = HashMap::new();
        while self.history.len() > 0 && (sequence_no - self.history[0].0) > CHAIN_GRACE {
            let item = self.history.remove(0);
            result.insert(item.2.clone(), (item.0 - item.3) as f32 * WEIGHT_COPYPASTA);
        }

        result
    }

    fn finish(&self) -> HashMap<String, f32> {
        self.finish()
    }
}

impl CopypastaLeader {
    fn lcs(s1: &str, s2: &str) -> String {
        let mut max: Option<String> = None; // Holds value of string with maximum length
        let mut current = String::new(); // String container to hold current longest value
        let mut s1_iter = s1.chars().peekable(); // Peekable iterator for string s1
        let mut s2_iter = s2.chars(); //Iterator for string s2
        let mut s2_prev_pos = s2_iter.clone(); // Iterator that holds position of previous location of first iterator
        let mut s1_prev_pos = s1_iter.clone(); // Peekable iterator used to make sure all possible combinations are located.
    
        loop {
            let s1_char = s1_iter.next(); // Get character in s1
    
            if current.is_empty() {
                // If no consecutive string found yet store location of iterator
                s1_prev_pos = s1_iter.clone()
            }
    
            match s1_char {
                Some(s1_char) => loop {
                    match s2_iter.next() {
                        Some(s2_char) if s1_char == s2_char => {
                            current.push(s1_char);
                            s2_prev_pos = s2_iter.clone();
                            break;
                        }
                        Some(_) => continue,
                        None => {
                            s2_iter = s2_prev_pos.clone();
                            break;
                        }
                    }
                },
                None => match s1_prev_pos.peek() {
                    Some(_) => {
                        if max.as_ref().map_or(true, |s| s.len() < current.len()) {
                            max = Some(current.clone());
                        }
                        current.clear();
    
                        s1_iter = s1_prev_pos.clone();
                        s2_iter = s2.chars();
                    }
                    None => break,
                },
            }
        }
    
        max.unwrap_or_default()
    }

    pub fn finish(&self) -> HashMap<String, f32> {
        self.history.iter()
            .map(|item| (item.2.clone(), (item.0 - item.3) as f32 * WEIGHT_COPYPASTA))
            .collect()
    }
}
