mod clip_id; // use stash_id module
pub use clip_id::ClipId; // export the StashId struct in stash_id module to clip directory(one level outer)

mod shortcode;
pub use shortcode::ShortCode;

mod content;
pub use content::Content;

mod title;
pub use title::Title;

mod posted;
pub use posted::Posted;

mod expires;
pub use expires::Expires;

mod password;
pub use password::Password;

mod hits;
pub use hits::Hits;

// vertical multi-cursor: cmd-option+ up or down
