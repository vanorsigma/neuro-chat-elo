use crate::_types::twitchtypes::Comment;
use crate::metrics::metrictrait::AbstractMetric;

use std::collections::{HashMap, BinaryHeap};

const WEIGHT_COPYPASTA: f32 = 0.3;
const CHAIN_GRACE: u32 = 10;
const MATCHING_THRESHOLD: f32 = 0.6;

pub struct CopypastaLeader {
    heap: BinaryHeap<(u32, String, String, u32)>,
}

impl AbstractMetric for CopypastaLeader {
    fn can_parallelize() -> bool {
        false
    }

    fn get_name() -> String {
        String::from("copypasta")
    }

    fn get_metric(&mut self, comment: &Comment, sequence_no: u32) -> HashMap<String, f32> {
        let text = comment.message.fragments.iter()
            .map(|fragment| fragment.text.clone())
            .collect::<Vec<String>>()
            .join(" ");

        if text.is_empty() {
            return HashMap::new();
        }

        // Evaluate or initialize the heap
        if self.heap.is_empty() {
            self.heap.push((sequence_no, text.clone(), comment.commenter._id.clone(), sequence_no));
        }

        // Find the best matching string in the heap
        let matching_scores: Vec<_> = self.heap.iter()
            .map(|item| {
                Self::lcs(&Self::pad_to(&item.1, &item.1, text.len()), &text) as f32 / text.len() as f32
            })
            .collect();

        if let Some(&max_score) = matching_scores.iter().max_by(|x, y| x.partial_cmp(y).unwrap()) {
            if max_score < MATCHING_THRESHOLD {
                self.heap.push((sequence_no, text, comment.commenter._id.clone(), sequence_no));
            }
        }

        // Evict old heap top
        let mut result = HashMap::new();
        while let Some(top) = self.heap.peek() {
            if sequence_no - top.0 > CHAIN_GRACE {
                let item = self.heap.pop().unwrap();
                result.insert(item.2.clone(), (item.0 - item.3) as f32 * WEIGHT_COPYPASTA);
            } else {
                break;
            }
        }

        result
    }

    fn finish(&self) -> HashMap<String, f32> {
        self.finish()
    }
}

impl CopypastaLeader {
    #[allow(dead_code)]
    pub fn new() -> Self {
        CopypastaLeader {
            heap: BinaryHeap::new(),
        }
    } 

    fn lcs(lhs: &str, rhs: &str) -> usize {
        let m = lhs.len();
        let n = rhs.len();
        let mut lcs_table = vec![vec![0; n + 1]; m + 1];

        for i in 0..=m {
            for j in 0..=n {
                if i == 0 || j == 0 {
                    lcs_table[i][j] = 0;
                } else if lhs.chars().nth(i - 1) == rhs.chars().nth(j - 1) {
                    lcs_table[i][j] = lcs_table[i - 1][j - 1] + 1;
                } else {
                    lcs_table[i][j] = usize::max(lcs_table[i - 1][j], lcs_table[i][j - 1]);
                }
            }
        }
        lcs_table[m][n]
    }

    fn pad_to(target: &str, padding: &str, maxlen: usize) -> String {
        let repeat_count = (maxlen - target.len()) / padding.len() + 1;
        let padded_string = padding.repeat(repeat_count);
        format!("{}{}", target, padded_string)[..maxlen].to_string()
    }

    pub fn finish(&self) -> HashMap<String, f32> {
        self.heap.iter()
            .map(|item| (item.2.clone(), (item.0 - item.3) as f32 * WEIGHT_COPYPASTA))
            .collect()
    }
}
