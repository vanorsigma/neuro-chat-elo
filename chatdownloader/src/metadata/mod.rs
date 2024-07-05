pub mod metadatatrait;
pub mod badges;
pub mod special_role;

use crate::twitch_utils::TwitchAPIWrapper;
use crate::metadata::metadatatrait::AbstractMetadata;

#[allow(dead_code)]
pub async fn get_metadata(twitch: TwitchAPIWrapper) -> Vec<Box<dyn AbstractMetadata>> {
    vec![
        Box::new(badges::Badges::new(twitch.clone()).await),
        Box::new(special_role::SpecialRole::new(twitch.clone()).await),
    ]
}