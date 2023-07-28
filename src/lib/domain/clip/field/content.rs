use crate::domain::clip::ClipError; // uses ClipError in field directory
use serde::{ Deserialize, Serialize };

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Content(String);

impl Content {
    pub fn new(content: &str) -> Result<Self, ClipError> {
        // there is content
        if !content.trim().is_empty() {
            Ok(Self(content.to_owned()))
        } else {
            Err(ClipError::EmptyContent)
        }
    }

    // turn(move ownership) self to String
    pub fn into_inner(self) -> String {
        self.0
    }
    // still keep a copy of self
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
