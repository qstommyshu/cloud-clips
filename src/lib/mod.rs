pub mod data;
pub mod domain;
pub mod service;
pub mod web;

// re-export
pub use domain::clip::field::ShortCode;
pub use domain::clip::ClipError;
pub use domain::time::Time;
pub use data::DataError;
