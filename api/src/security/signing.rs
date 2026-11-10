use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

fn mac(secret: &[u8], parts: &[&str]) -> HmacSha256 {
    let mut mac = HmacSha256::new_from_slice(secret).expect("HMAC accepts any key length");
    mac.update(parts.join("\n").as_bytes());
    mac
}

/// HMAC-SHA256 over the parts, joined with newlines, as lowercase hex.
pub fn sign(secret: &[u8], parts: &[&str]) -> String {
    hex::encode(mac(secret, parts).finalize().into_bytes())
}

/// Constant-time check of a signature made by [`sign`].
pub fn verify(secret: &[u8], parts: &[&str], signature: &str) -> bool {
    match hex::decode(signature) {
        Ok(expected) => mac(secret, parts).verify_slice(&expected).is_ok(),
        Err(_) => false,
    }
}

/// A salted, one-way identifier for an IP address for one day. Raw addresses are never stored.
pub fn ip_hash(secret: &[u8], ip: &str, day: u64) -> String {
    sign(secret, &["ip", ip, &day.to_string()])[..32].to_string()
}

/// Like [`ip_hash`] but also tells browsers behind the same address apart.
pub fn visitor_hash(secret: &[u8], ip: &str, user_agent: &str, day: u64) -> String {
    sign(secret, &["visitor", ip, user_agent, &day.to_string()])[..32].to_string()
}
