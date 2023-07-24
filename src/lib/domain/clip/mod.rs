pub mod field;

use serde::{ Deserialize, Serialize };
use thiserror::Error;

pub enum ClipError {
    // attribute macro provided by `thiserror`, used to define a custom error msg for error
    #[error("invalid password: {0}")] InvalidPassword(String),
    #[error("invalid title: {0}")] InvalidTitle(String),
    #[error("empty content: {0}")]
    EmptyContent,
    #[error("invalid date: {0}")] InvalidDate(String),
    // attribute macro provided by `thiserror`, DateParse error should be automatically constructed from chrono::ParserError
    #[error("invalid password: {0}")] DateParse(#[from] chrono::ParserError),
    #[error("invalid id: {0}")] Id(#[from] uuid::Error),
    #[error("hit parse error: {0}")] Hits(#[from] std::num::TryFromIntError),
}

#[derive(Debud, Clone, Deserialize, Serialize)]
pub struct Clip {
    pub clip_id: field::ClipId,
    pub shortcode: field::ShortCode,
    pub content: field::Content,
    pub title: field::Title,
    pub posted: field::Posted,
    pub expires: field::Expires,
    pub password: field::Password,
    pub hits: field::Hits,
}
