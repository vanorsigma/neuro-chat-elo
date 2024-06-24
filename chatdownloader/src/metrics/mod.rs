pub mod metrictrait;
pub mod bits;
pub mod subs;
pub mod text;
pub mod copypastaleader;
pub mod emote;

use crate::metrics::metrictrait::AbstractMetric;

pub async fn get_metrics() -> Vec<Box<dyn AbstractMetric>> {
    vec![
        Box::new(bits::Bits::new().await),
        Box::new(subs::Subs::new().await),
        Box::new(text::Text::new().await),
        Box::new(copypastaleader::CopypastaLeader::new().await),
        Box::new(emote::Emote::new().await),
    ]
}