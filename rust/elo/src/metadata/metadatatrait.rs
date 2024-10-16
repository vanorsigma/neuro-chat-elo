//! Represents an abstract metadata
use crate::_types::clptypes::{Message, MetadataTypes, MetadataUpdate};

pub trait AbstractMetadata: Sized {
    /*
    Structs that implement this trait represent a piece of metadata

    Struct should ensure to set self.twtich to the twitch object passed
    if it needs to make API calls
    */

    /// Name of this piece of metadata
    fn get_name(&self) -> String;

    /// Get the defautl value for this metadata
    fn get_default_value(&self) -> MetadataTypes;

    /// Get information about a user from a chat message
    fn get_metadata(&self, message: Message, sequence_no: u32) -> MetadataUpdate;
}
