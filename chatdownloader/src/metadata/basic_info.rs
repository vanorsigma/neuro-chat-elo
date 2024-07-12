/*
Get the username and avatar of the user
*/

use std::collections::HashMap;

use crate::_types::clptypes::MetadataTypes;
use crate::_types::twitchtypes::Comment;
use crate::metadata::metadatatrait::AbstractMetadata;
use crate::twitch_utils::TwitchAPIWrapper;

#[derive(Default, Debug)]
pub struct BasicInfo;

impl AbstractMetadata for BasicInfo {
    /*
    Figures out if the user is a special role
    */

    async fn new(_twitch: &TwitchAPIWrapper) -> Self {
        Self
    }

    fn get_name(&self) -> String {
        "basic_info".to_string()
    }

    fn get_default_value(&self) -> MetadataTypes {
        MetadataTypes::BasicInfo("".to_string(), "".to_string())
    }

    fn get_metadata(
        &self,
        comment: Comment,
        _sequence_no: u32,
    ) -> (String, HashMap<String, MetadataTypes>) {
        let mut metadata: HashMap<String, MetadataTypes> = HashMap::new();
        metadata.insert(
            comment.commenter._id.clone(),
            MetadataTypes::BasicInfo(
                comment.commenter.display_name.clone(),
                comment.commenter.logo.clone(),
            ),
        );
        (self.get_name(), metadata)
    }
}
