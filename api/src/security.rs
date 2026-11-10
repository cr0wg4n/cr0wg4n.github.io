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
    if ALLOWED_PREFIXES.iter().any(|p| path.starts_with(p) && path.len() > p.len()) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signature_roundtrip_and_tamper() {
        let sig = sign(b"k", &["approve", "/posts/a/", "C#1"]);
        assert!(verify(b"k", &["approve", "/posts/a/", "C#1"], &sig));
        assert!(!verify(b"k", &["delete", "/posts/a/", "C#1"], &sig));
        assert!(!verify(b"other", &["approve", "/posts/a/", "C#1"], &sig));
        assert!(!verify(b"k", &["approve", "/posts/a/", "C#1"], "zz"));
        assert!(!verify(b"k", &["approve", "/posts/a/", "C#1"], ""));
    }

    #[test]
    fn visitor_hash_changes_with_day_and_input() {
        let a = visitor_hash(b"k", "1.2.3.4", "ua", 1);
        assert_eq!(a.len(), 32);
        assert_eq!(a, visitor_hash(b"k", "1.2.3.4", "ua", 1));
        assert_ne!(a, visitor_hash(b"k", "1.2.3.4", "ua", 2));
        assert_ne!(a, visitor_hash(b"k", "1.2.3.5", "ua", 1));
    }

    #[test]
    fn paths() {
        assert_eq!(normalize_path("/posts/my-post"), Some("/posts/my-post/".into()));
        assert_eq!(normalize_path("/es/posts/mi_post/"), Some("/es/posts/mi_post/".into()));
        assert_eq!(normalize_path("/posts/"), None);
        assert_eq!(normalize_path("/talks/x/"), None);
        assert_eq!(normalize_path("/posts/../etc/"), None);
        assert_eq!(normalize_path("/posts/a?b=1"), None);
        assert_eq!(normalize_path("/posts//a/"), None);
        assert_eq!(normalize_path("https://evil.example/posts/a/"), None);
        assert_eq!(normalize_path(""), None);
        assert_eq!(normalize_path(&format!("/posts/{}", "a".repeat(300))), None);
    }

    #[test]
    fn text_cleaning() {
        assert_eq!(clean_text("  hola \r\n mundo  ", 50, true), Some("hola \n mundo".into()));
        assert_eq!(clean_text("a\u{0007}b", 50, false), Some("ab".into()));
        assert_eq!(clean_text("a\nb", 50, false), Some("ab".into()));
        assert_eq!(clean_text("   ", 50, true), None);
        assert_eq!(clean_text("ñññ", 2, true), None);
        assert_eq!(clean_text("ññ", 2, true), Some("ññ".into()));
    }

    #[test]
    fn escaping() {
        assert_eq!(html_escape("<a href=\"x\">&'</a>"), "&lt;a href=&quot;x&quot;&gt;&amp;&#39;&lt;/a&gt;");
    }
}
