//! Pure helpers (no AWS): signing and hashing, input validation, HTML escaping.

mod escape;
mod signing;
mod token;
mod validation;

pub use escape::html_escape;
pub use signing::{ip_hash, sign, verify, visitor_hash};
pub use token::{decode_token, encode_token};
pub use validation::{clean_text, normalize_page};
