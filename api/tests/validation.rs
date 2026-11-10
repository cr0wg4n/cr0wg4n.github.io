//! What the API accepts as the identifier of a post and as comment text.

use comments_api::security::{clean_text, normalize_page};

#[test]
fn every_form_of_the_same_post_gives_the_same_key() {
    let key = Some("posts/my-post".to_string());
    assert_eq!(
        normalize_page("/posts/my-post"),
        key,
        "Hugo's translation key"
    );
    assert_eq!(normalize_page("posts/my-post"), key);
    assert_eq!(
        normalize_page("/posts/my-post/"),
        key,
        "English page address"
    );
    assert_eq!(
        normalize_page("/es/posts/my-post/"),
        key,
        "Spanish page address"
    );
    assert_eq!(normalize_page("es/posts/my-post"), key);
    assert_eq!(normalize_page("  /posts/my-post  "), key);
}

#[test]
fn accepts_real_slugs() {
    assert_eq!(
        normalize_page("/posts/AI-slop"),
        Some("posts/AI-slop".into())
    );
    assert_eq!(
        normalize_page("/es/posts/mi_post/"),
        Some("posts/mi_post".into())
    );
    assert_eq!(
        normalize_page("/posts/masters-of-sampling-miloj-daft-punk"),
        Some("posts/masters-of-sampling-miloj-daft-punk".into())
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
        "/projects/x",
        "/posts/a/b/",
        "/posts/../etc/",
        "/posts/a.b/",
        "/posts/a?b=1",
        "/posts/a#top",
        "/posts//a/",
        "/posts/a b/",
        "/posts/á/",
        "/fr/posts/a/",
        "https://evil.example/posts/a/",
        "//evil.example/posts/a/",
        "es/es/posts/a",
    ] {
        assert_eq!(normalize_page(bad), None, "should reject {bad:?}");
    }
    assert_eq!(
        normalize_page(&format!("/posts/{}", "a".repeat(121))),
        None,
        "slug too long"
    );
    assert_eq!(
        normalize_page(&format!("/posts/{}", "a".repeat(300))),
        None,
        "key too long"
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
