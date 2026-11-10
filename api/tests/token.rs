//! The token in moderation links. It must survive anything a browser or mail app does to a URL.

use comments_api::security::{decode_token, encode_token};

#[test]
fn roundtrip() {
    let (pk, sk) = (
        "P#posts/masters-of-sampling-miloj-daft-punk",
        "C#1794288401890#2fa03f2206cf443ba83eb8190cef9aac",
    );
    assert_eq!(
        decode_token(&encode_token(pk, sk)),
        Some((pk.to_string(), sk.to_string()))
    );
}

#[test]
fn token_has_no_character_that_a_browser_could_decode_or_cut() {
    // "#" starts a URL fragment, "%" and "+" are decoded, "/" "?" "&" "=" have meaning: none may appear.
    let token = encode_token(
        "P#posts/a-b_c",
        "C#1794288401890#2fa03f2206cf443ba83eb8190cef9aac",
    );
    assert!(
        token
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_'),
        "unsafe character in {token}"
    );
}

#[test]
fn rejects_what_is_not_a_token() {
    for bad in ["", "%%%", "not base64!", "YQ", &"A".repeat(401)] {
        assert_eq!(decode_token(bad), None, "should reject {bad:?}");
    }
}
