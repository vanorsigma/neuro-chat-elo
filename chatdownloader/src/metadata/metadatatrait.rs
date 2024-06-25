/*
Represents an abstract metadata
*/

use std::collections::HashMap;
use crate::_types::twitchtypes::Comment;
use crate::_types::clptypes::MetadataTypes;
use crate::twitch_utils::TwitchAPIWrapper;

#[allow(dead_code)]
pub trait AbstractMetadata {
    /*
    Structs that implement this trait represent a piece of metadata

    Struct should ensure to set self.twtich to the twitch object passed
    if it needs to make API calls
    */

    #[allow(dead_code)]
    async fn new(twitch: TwitchAPIWrapper) -> Self where Self: Sized;
        /*
        Create a new metadata object

        :param twitch: A TwitchAPIWrapper object
        */

    #[allow(dead_code)]
    fn get_name(&self) -> String;
        /*
        Name of this piece of metadata
        */
        
    #[allow(dead_code)]
    fn get_default_value(&self) -> MetadataTypes;
        /*
        Get the default value for this metadata
        */

    #[allow(dead_code)]
    fn get_metadata(&self, comment: Comment, sequence_no: u32) -> HashMap<String, MetadataTypes>;
        /*
        Get information about a user from a chat message

        :param: comment A comment from the user
        :returns: A partial update to a user's metadata (dictionary of
                  username to value)
        */
}