//! What the API accepts as a page address and as comment text.

use comments_api::security::{clean_text, normalize_path};

#[test]
fn accepts_one_post_in_either_language() {
    assert_eq!(
        normalize_path("/posts/my-post"),
        Some("/posts/my-post/".into())
    );
    assert_eq!(
        normalize_path("/posts/my-post/"),
        Some("/posts/my-post/".into())
    );
    assert_eq!(
        normalize_path("/es/posts/mi_post/"),
        Some("/es/posts/mi_post/".into())
    );
    assert_eq!(
        normalize_path("/posts/AI-slop/"),
        Some("/posts/AI-slop/".into())
    );
    assert_eq!(
        normalize_path("/posts/masters-of-sampling-miloj-daft-punk/"),
        Some("/posts/masters-of-sampling-miloj-daft-punk/".into())
    );
}

#[test]
fn rejects_everything_else() {
    for bad in [
        "",
        "/",
        "/posts/",
        "/es/posts/",
        "/talks/x/",
        "/posts/a/b/",
        "/posts/../etc/",
        "/posts/a.b/",
        "/posts/a?b=1",
        "/posts/a#top",
        "/posts//a/",
        "/posts/a b/",
        "/posts/á/",
        "https://evil.example/posts/a/",
        "//evil.example/posts/a/",
        "posts/a/",
    ] {
        assert_eq!(normalize_path(bad), None, "should reject {bad:?}");
    }
    assert_eq!(
        normalize_path(&format!("/posts/{}", "a".repeat(121))),
        None,
        "slug too long"
    );
    assert_eq!(
        normalize_path(&format!("/posts/{}", "a".repeat(300))),
        None,
        "path too long"
    );
}

#[test]
fn text_is_trimmed_and_cleaned() {
    assert_eq!(
        clean_text("  hola \r\n mundo  ", 50, true),
        Some("hola \n mundo".into())
    );
    assert_eq!(
        clean_text("a\u{0007}b", 50, false),
        Some("ab".into()),
        "control characters dropped"
    );
    assert_eq!(
        clean_text("a\nb", 50, false),
        Some("ab".into()),
        "newlines dropped when not allowed"
    );
    assert_eq!(clean_text("   ", 50, true), None, "only spaces");
    assert_eq!(clean_text("", 50, true), None);
}

#[test]
fn length_counts_characters_not_bytes() {
    assert_eq!(clean_text("ñññ", 2, true), None);
    assert_eq!(clean_text("ññ", 2, true), Some("ññ".into()));
    assert_eq!(clean_text("👍👍", 2, true), Some("👍👍".into()));
}
