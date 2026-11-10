//! Pure helpers: signing, hashing, validation and escaping. No AWS here, so everything is unit-tested.

use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Section prefixes where comments and reactions are accepted (Hugo: /posts/ and /es/posts/).
const ALLOWED_PREFIXES: [&str; 2] = ["/posts/", "/es/posts/"];

/// HMAC-SHA256 over the parts, joined with newlines, as lowercase hex.
pub fn sign(secret: &[u8], parts: &[&str]) -> String {
    let mut mac = HmacSha256::new_from_slice(secret).expect("HMAC accepts any key length");
    mac.update(parts.join("\n").as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

/// Constant-time check of a signature made by [`sign`].
pub fn verify(secret: &[u8], parts: &[&str], signature: &str) -> bool {
    let Ok(expected) = hex::decode(signature) else {
        return false;
    };
    let mut mac = HmacSha256::new_from_slice(secret).expect("HMAC accepts any key length");
    mac.update(parts.join("\n").as_bytes());
    mac.verify_slice(&expected).is_ok()
}

/// A salted, one-way identifier for a visitor for one day. Raw IPs are never stored.
pub fn visitor_hash(secret: &[u8], ip: &str, user_agent: &str, day: u64) -> String {
    sign(secret, &["visitor", ip, user_agent, &day.to_string()])[..32].to_string()
}

/// Accepts "/posts/my-post/" style paths of a known section and returns them in canonical form
/// (leading and trailing slash, no query or fragment). Anything else is rejected.
pub fn normalize_path(raw: &str) -> Option<String> {
    let mut path = raw.trim().to_string();
    if path.len() > 200 || path.is_empty() {
        return None;
    }
    if !path.ends_with('/') {
        path.push('/');
    }
    let ok_chars = path
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '/' | '-' | '_' | '.'));
    if !ok_chars || path.contains("..") || path.contains("//") {
        return None;
    }
    if ALLOWED_PREFIXES
        .iter()
        .any(|p| path.starts_with(p) && path.len() > p.len())
    {
        Some(path)
    } else {
        None
    }
}

/// Trims, normalizes line breaks, drops control characters and enforces 1..=max characters.
pub fn clean_text(raw: &str, max_chars: usize, allow_newlines: bool) -> Option<String> {
    let text: String = raw
        .replace("\r\n", "\n")
        .chars()
        .filter(|c| !c.is_control() || (allow_newlines && *c == '\n'))
        .collect();
    let text = text.trim().to_string();
    let n = text.chars().count();
    if n == 0 || n > max_chars {
        None
    } else {
        Some(text)
    }
}

pub fn html_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}
