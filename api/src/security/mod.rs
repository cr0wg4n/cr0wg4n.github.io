//! Pure helpers (no AWS): signing and hashing, input validation, HTML escaping.

mod escape;
mod signing;
mod validation;

pub use escape::html_escape;
pub use signing::{ip_hash, sign, verify, visitor_hash};
pub use validation::{clean_text, normalize_path};
