//! Tests for the pure helpers in src/security.rs: signing, hashing, validation and escaping.

use comments_api::security::{clean_text, html_escape, normalize_path, sign, verify, visitor_hash};

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
    assert_eq!(
        normalize_path("/posts/my-post"),
        Some("/posts/my-post/".into())
    );
    assert_eq!(
        normalize_path("/es/posts/mi_post/"),
        Some("/es/posts/mi_post/".into())
    );
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
    assert_eq!(
        clean_text("  hola \r\n mundo  ", 50, true),
        Some("hola \n mundo".into())
    );
    assert_eq!(clean_text("a\u{0007}b", 50, false), Some("ab".into()));
    assert_eq!(clean_text("a\nb", 50, false), Some("ab".into()));
    assert_eq!(clean_text("   ", 50, true), None);
    assert_eq!(clean_text("ñññ", 2, true), None);
    assert_eq!(clean_text("ññ", 2, true), Some("ññ".into()));
}

#[test]
fn escaping() {
    assert_eq!(
        html_escape("<a href=\"x\">&'</a>"),
        "&lt;a href=&quot;x&quot;&gt;&amp;&#39;&lt;/a&gt;"
    );
}
