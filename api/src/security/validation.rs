/// Accepts the address of one post, "/posts/<slug>/" or "/es/posts/<slug>/", and returns it in canonical form
/// (with the trailing slash). Anything else is rejected: other sections, nested paths, queries, odd characters.
pub fn normalize_path(raw: &str) -> Option<String> {
    let mut path = raw.trim().to_string();
    if path.is_empty() || path.len() > 200 {
        return None;
    }
    if !path.ends_with('/') {
        path.push('/');
    }
    let rest = path
        .strip_prefix("/es/posts/")
        .or_else(|| path.strip_prefix("/posts/"))?;
    let slug = rest.strip_suffix('/')?;
    let valid = !slug.is_empty()
        && slug.len() <= 120
        && slug
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
    valid.then_some(path)
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
