/// The identifier of one post, shared by all its translations: "posts/<slug>".
///
/// Accepts the key Hugo gives to every language version of a page ("/posts/<slug>"), and also the address of either
/// language version ("/posts/<slug>/", "/es/posts/<slug>/"), so the English and the Spanish page of a post end up
/// with the same comments and reactions. Anything else is rejected: other sections, nested paths, queries, odd characters.
pub fn normalize_page(raw: &str) -> Option<String> {
    let raw = raw.trim();
    if raw.is_empty() || raw.len() > 200 {
        return None;
    }
    let raw = raw.strip_prefix('/').unwrap_or(raw);
    let raw = raw.strip_suffix('/').unwrap_or(raw);
    let raw = raw.strip_prefix("es/").unwrap_or(raw);
    let slug = raw.strip_prefix("posts/")?;
    let valid = !slug.is_empty()
        && slug.len() <= 120
        && slug
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
    valid.then(|| format!("posts/{slug}"))
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
    (1..=max_chars).contains(&n).then_some(text)
}
