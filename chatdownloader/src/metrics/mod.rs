mod metrictrait;
pub mod bits;
pub mod subs;
pub mod text;
pub mod copypastaleader;

use std::collections::HashMap;
use crate::_types::twitchtypes::Comment;
use crate::metrics::metrictrait::AbstractMetric;

pub struct Metrics {
    metrics: Vec<Box<dyn AbstractMetric>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            metrics: vec![
                Box::new(bits::Bits),
                Box::new(subs::Subs),
                Box::new(text::Text),
                Box::new(copypastaleader::CopyPastaLeader),
            ],
        }
    }

    pub fn get_metrics(&self) -> Vec<String> {
        self.metrics.iter().map(|m| m.get_name()).collect()
    }

    pub fn get_metric(&mut self, comment: &Comment, sequence_no: u32) -> HashMap<String, f32> {
        let mut metric_map: HashMap<String, f32> = HashMap::new();
        for metric in self.metrics.iter_mut() {
            let metric_name = metric.get_name();
            let metric_value = metric.get_metric(comment, sequence_no);
            metric_map.insert(metric_name, metric_value);
        }
        metric_map
    }

    pub fn finish(&mut self) -> HashMap<String, f32> {
        let mut metric_map: HashMap<String, f32> = HashMap::new();
        for metric in self.metrics.iter_mut() {
            let metric_name = metric.get_name();
            let metric_value = metric.finish();
            metric_map.insert(metric_name, metric_value);
        }
        metric_map
    }
}