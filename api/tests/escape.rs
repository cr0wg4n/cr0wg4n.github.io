use comments_api::security::html_escape;

#[test]
fn escapes_everything_that_can_start_markup() {
    assert_eq!(
        html_escape("<a href=\"x\">&'</a>"),
        "&lt;a href=&quot;x&quot;&gt;&amp;&#39;&lt;/a&gt;"
    );
    assert_eq!(
        html_escape("<script>alert(1)</script>"),
        "&lt;script&gt;alert(1)&lt;/script&gt;"
    );
    assert_eq!(html_escape("plain text ñ"), "plain text ñ");
}
