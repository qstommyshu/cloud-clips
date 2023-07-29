use crate::domain::clip::ClipError;
use serde::{ Deserialize, Serialize };
use std::str::FromStr;
use derive_more::From;

#[derive(Debug, Clone, Deserialize, Serialize, From)]
pub struct ShortCode(String);

impl ShortCode {
    pub fn new() -> Self {
        use rand::prelude::*;
        let allowed_chars = ['a', 'b', 'c', 'd', '1', '2', '3', '4'];

        let mut rng = thread_rng();
        let mut shortcode = String::with_capacity(10);
        for _ in 0..10 {
            shortcode.push(
                *allowed_chars.choose(&mut rng).expect("sampling array should have values")
            );
        }
        Self(shortcode)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}
// Default provides a constructor Default::default() to construct ShortCode with customized default value
impl Default for ShortCode {
    fn default() -> Self {
        Self::new()
    }
}

// traits for ShortCode -> String
impl From<ShortCode> for String {
    fn from(shortcode: ShortCode) -> Self {
        shortcode.0
    }
}
// get a &str from a web request, then &str.into() to get a ShortCode
impl From<&str> for ShortCode {
    fn from(shortcode: &str) -> Self {
        ShortCode(shortcode.to_owned())
    }
}

// TODO: explain why we need it? test
// From Str is similar to From<&str>
impl FromStr for ShortCode {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.into()))
    }
}
