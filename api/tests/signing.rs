//! Signatures for the moderation links, and the one-way visitor identifiers.

use comments_api::security::{ip_hash, sign, verify, visitor_hash};

#[test]
fn signature_roundtrip_and_tamper() {
    let sig = sign(b"k", &["approve", "P#/posts/a/", "C#1"]);
    assert!(verify(b"k", &["approve", "P#/posts/a/", "C#1"], &sig));
    assert!(
        !verify(b"k", &["delete", "P#/posts/a/", "C#1"], &sig),
        "action is part of the signature"
    );
    assert!(
        !verify(b"k", &["approve", "P#/posts/b/", "C#1"], &sig),
        "page is part of the signature"
    );
    assert!(
        !verify(b"other", &["approve", "P#/posts/a/", "C#1"], &sig),
        "wrong secret"
    );
    assert!(
        !verify(b"k", &["approve", "P#/posts/a/", "C#1"], "zz"),
        "not hex"
    );
    assert!(
        !verify(b"k", &["approve", "P#/posts/a/", "C#1"], ""),
        "empty"
    );
}

#[test]
fn parts_cannot_be_shifted_between_fields() {
    // "a\nb" + "c" must not verify as "a" + "b\nc" ... the newline separator is rejected inside parts by design:
    let sig = sign(b"k", &["a", "bc"]);
    assert!(!verify(b"k", &["ab", "c"], &sig));
}

#[test]
fn ip_hash_is_stable_per_day_and_hides_the_address() {
    let a = ip_hash(b"k", "203.0.113.7", 1);
    assert_eq!(a.len(), 32);
    assert_eq!(a, ip_hash(b"k", "203.0.113.7", 1));
    assert_ne!(a, ip_hash(b"k", "203.0.113.7", 2), "changes every day");
    assert_ne!(a, ip_hash(b"k", "203.0.113.8", 1));
    assert_ne!(
        a,
        ip_hash(b"other", "203.0.113.7", 1),
        "depends on the secret"
    );
    assert!(
        !a.contains("203"),
        "the address is not recoverable from the hash text"
    );
}

#[test]
fn visitor_hash_tells_browsers_apart_but_ip_hash_does_not() {
    let a = visitor_hash(b"k", "203.0.113.7", "Firefox", 1);
    let b = visitor_hash(b"k", "203.0.113.7", "Chrome", 1);
    assert_ne!(a, b);
    assert_ne!(a, ip_hash(b"k", "203.0.113.7", 1));
}
