/*
Represents an abstract metadata
*/

use std::any;
use std::collections::HashMap;
use crate::_types::twitchtypes::Comment;
use crate::twitch_utils::TwitchAPIWrapper;

pub trait AbstractMetadata {
    /*
    Structs that implement this trait represent a piece of metadata

    Struct should ensure to set self.twtich to the twitch object passed
    if it needs to make API calls
    */
    type MetadataType; // The type the metadata stores

    fn new(twitch: TwitchAPIWrapper) -> Self;
        /*
        Create a new metadata object

        :param twitch: A TwitchAPIWrapper object
        */

    fn get_name() -> String;
        /*
        Name of this piece of metadata
        */

    fn get_metadata(&self, comment: Comment, sequence_no: u32) -> HashMap<String, Self::MetadataType>;
        /*
        Get information about a user from a chat message

        :param: comment A comment from the user
        :returns: A partial update to a user's metadata (dictionary of
                  username to value)
        */
}