//! Moderation links carry the comment's identity as one token made only of letters, digits, "-" and "_".
//!
//! The identity ("P#posts/<slug>", "C#<ms>#<id>") contains "#" and "/". Percent-encoded in a link they
//! survive most browsers, but some (Firefox on Android) decode "%23" back to "#" and then drop everything after it
//! as a URL fragment, which cut the link in half. A token needs no encoding, so nothing can alter it on the way.

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

const MAX_TOKEN_CHARS: usize = 400;

pub fn encode_token(pk: &str, sk: &str) -> String {
    URL_SAFE_NO_PAD.encode(format!("{pk}\n{sk}"))
}

/// The (PK, SK) inside a token, or None if it is not one made by [`encode_token`].
pub fn decode_token(token: &str) -> Option<(String, String)> {
    if token.is_empty() || token.len() > MAX_TOKEN_CHARS {
        return None;
    }
    let bytes = URL_SAFE_NO_PAD.decode(token).ok()?;
    let text = String::from_utf8(bytes).ok()?;
    let (pk, sk) = text.split_once('\n')?;
    Some((pk.to_string(), sk.to_string()))
}
