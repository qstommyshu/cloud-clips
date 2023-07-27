use chrono::{ DateTime, NaiveDateTime, Utc };
use derive_more::From;
use serde::{ Deserialize, Serialize };
use std::str::FromStr;

// What does derive_more From does?
#[derive(Clone, Debug, From, Deserialize, Serialize)]
// Benefit of wrapping DateTime into Time Struct: breaking update in DateTime trait will can be
// limited to change only in Time struct we created, and we can use derive to add traits to Time.
pub struct Time(DateTime<Utc>);

impl Time {
    pub fn into_inner(self) -> DateTime<Utc> {
        self.0
    }

    pub fn timestamp(&self) -> i64 {
        self.0.timestamp()
    }

    pub fn from_naive_utc(datetime: NaiveDateTime) -> Self {
        Time(DateTime::from_utc(datetime, Utc))
    }
}

impl FromStr for Time {
    type Err = chrono::ParseError;
    // This is the only location &str -> Time is allowed to happen
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 2022-05-22 to DateTime struct
        match format!("{s}T00:00:00Z").parse::<DateTime<Utc>>() {
            Ok(time) => Ok(time.into()),
            Err(e) => Err(e), // chrono::ParseError
        }
    }
}
