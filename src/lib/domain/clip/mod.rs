pub mod field;

use serde::{ Deserialize, Serialize };
use thiserror::Error;

#[derive(Debud, Clone, Deserialize, Serialize)]
pub struct Stash {
    pub stash_id: field::StashId,
    pub shortcode: field::ShortCode,
    pub content: field::Content,
    pub title: field::Title,
    pub posted: field::Posted,
    pub expires: field::Expires,
    pub password: field::Password,
    pub hits: field::Hits,
}
