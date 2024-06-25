pub mod metrictrait;
pub mod bits;
pub mod subs;
pub mod text;
pub mod copypastaleader;

use crate::metrics::metrictrait::AbstractMetric;

pub fn get_metrics() -> Vec<Box<dyn AbstractMetric>> {
    vec![
        Box::new(bits::Bits::new()),
        Box::new(subs::Subs::new()),
        Box::new(text::Text::new()),
        Box::new(copypastaleader::CopypastaLeader::new()),
    ]
}