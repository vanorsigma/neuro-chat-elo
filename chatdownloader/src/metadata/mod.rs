pub mod metadatatrait;
pub mod badges;
pub mod special_role;

use crate::twitch_utils::TwitchAPIWrapper;
use crate::metadata::metadatatrait::AbstractMetadata;

/*
TODO: FIX THIS FR FR PLEASE THIS IS TERRIBLE AND I HATE IT REEEEEEEEEEEEEEEEEEEEEEE
*/

#[allow(dead_code)]
pub async fn get_metadata(twitch: TwitchAPIWrapper) -> Vec<Box<dyn AbstractMetadata>> {
    let mut metadatas: Vec<Box<dyn AbstractMetadata>> = vec![];
    let special_role = special_role::SpecialRole::new(twitch.clone()).await;
    let badges = badges::Badges::new(twitch.clone()).await;
    metadatas.push(Box::new(special_role));
    metadatas.push(Box::new(badges));
    metadatas
}