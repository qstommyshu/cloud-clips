use crate::domain::clip::ClipError;
use serde::{ Deserialize, Serialize };
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Title(Option<String>);

// optional title for clips
impl Title {
    // Title can be either None or Some title, won't create error at all
    pub fn new<T: Into<Option<String>>>(title: T) -> Self {
        // TODO: explain .into()
        let title: Option<String> = title.into();
        match title {
            // if user provide something in title field
            Some(title) => {
                // have characters after trim
                if !title.trim().is_empty() {
                    Self(Some(title))
                    // TODO: add title complexity rules, use ClipError
                } else {
                    // only spaces are provided
                    Self(None)
                }
            }
            // no title provided
            None => Self(None),
        }
    }

    pub fn into_inner(self) -> Option<String> {
        self.0
    }

    pub fn has_password(&self) -> bool {
        self.0.is_some()
    }
}

impl Default for Title {
    fn default() -> Self {
        Self::new(None)
    }
}

impl FromStr for Title {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string()))
    }
}
