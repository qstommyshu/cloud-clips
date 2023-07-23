pub mod field;

use serde::{ Deserialize, Serialize };
use thiserror::Error;

pub enum ClipError {
    //TODO: explain #[error()] syntax
    #[error("invalid password: {0}")] InvalidPassword(String),
    #[error("invalid title: {0}")] InvalidTitle(String),
    #[error("empty content: {0}")]
    EmptyContent,
    #[error("invalid date: {0}")] InvalidDate(String),
    //TODO: explain #[from] syntax
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
