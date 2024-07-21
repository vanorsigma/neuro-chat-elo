/*
Represents an abstract metadata
*/

use crate::_types::clptypes::{MetadataTypes, MetadataUpdate};
use crate::_types::twitchtypes::Comment;
use crate::twitch_utils::TwitchAPIWrapper;

pub trait AbstractMetadata: Sized {
    /*
    Structs that implement this trait represent a piece of metadata

    Struct should ensure to set self.twtich to the twitch object passed
    if it needs to make API calls
    */

    async fn new(twitch: &TwitchAPIWrapper) -> Self
    where
        Self: Sized + Send;
    /*
    Create a new metadata object

    :param twitch: A TwitchAPIWrapper object
    */

    fn get_name(&self) -> String;
    /*
    Name of this piece of metadata
    */

    fn get_default_value(&self) -> MetadataTypes;
    /*
    Get the default value for this metadata
    */

    fn get_metadata(
        &self,
        comment: Comment,
        sequence_no: u32,
    ) -> MetadataUpdate;
    /*
    Get information about a user from a chat message

    :param: comment A comment from the user
    :returns: A partial update to a user's metadata (dictionary of
              username to value)
    */
}
