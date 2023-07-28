use crate::domain::time::Time;
use crate::domain::clip::ClipError;
use serde::{ Deserialize, Serialize };
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Expires(Option<Time>);

impl Expires {
    pub fn new<T: Into<Option<Time>>>(expires: T) -> Self {
        Self(expires.into())
    }

    pub fn into_inner(self) -> Option<Time> {
        self.0
    }
}

// Default trait allows `Default::default()` constructor that creates some default value.
impl Default for Expires {
    fn default() -> Self {
        Self::new(None)
    }
}

// FromStr trait allows to create Expires Struct using &str
impl FromStr for Expires {
    type Err = ClipError;
    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        // if &str is empty, create empty Expires
        if raw.is_empty() {
            Ok(Self(None))
        } else {
            // Time::from_str creates a Time struct using &str
            match Time::from_str(raw) {
                // if Ok, use Time struct to create a Expires
                Ok(time) => Ok(Self::new(time)),
                // can't parse &str to time, wrong format(defined in time.rs)
                // e.into() turn ParseError into a ClipError, #[from] for chrono::ParseError
                // defined in: /Users/tommy/Developers/cloud-clips/src/lib/domain/clip/mod.rs
                Err(e) => Err(e.into()),
            }
        }
    }
}
