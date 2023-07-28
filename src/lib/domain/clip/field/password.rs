use crate::domain::clip::ClipError;
use serde::{ Deserialize, Serialize };
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Password(Option<String>); // Option so password is optional

impl Password {
    pub fn new<T: Into<Option<String>>>(password: T) -> Result<Self, ClipError> {
        // TODO: explain .into() why do we need into() here? So that we can provide str or None, not Some(str) or None?
        // Into<Option<String>> means a type that implments Into<Option<String>>? need to check if &str is Into<Option<String>>
        let password: Option<String> = password.into();
        match password {
            // if user provide something in password field
            Some(password) => {
                // have characters after trim
                if !password.trim().is_empty() {
                    Ok(Self(Some(password)))
                    // TODO: add password complexity rules, use ClipError
                } else {
                    // only spaces are provided
                    Ok(Self(None))
                }
            }
            // no password provided
            None => Ok(Self(None)),
        }
    }

    pub fn into_inner(self) -> Option<String> {
        self.0
    }

    pub fn has_password(&self) -> bool {
        self.0.is_some()
    }
}

impl Default for Password {
    fn default() -> Self {
        // default password are empty
        Self(None)
    }
}

impl FromStr for Password {
    type Err = ClipError;
    // doesn't consume the input string
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}
