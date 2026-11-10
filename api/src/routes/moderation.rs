//! The delete link from the new-comment e-mail. A link only opens a confirmation page (mail scanners open links);
//! the change happens when the button on that page is pressed.
//!
//! Link: /moderate?a=delete&t=<token>&s=<signature>. The token holds the comment's key (security/token.rs).

use crate::config::{App, MAX_BODY_BYTES};
use crate::http::{body_bytes, html, json};
use crate::security::{decode_token, html_escape, verify};
use lambda_http::{Body, Error, Request, RequestExt, Response};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct Form {
    a: String,
    t: String,
    s: String,
}

/// Checks a link and returns the comment it points at. A rejected link is logged with the reason (never the
/// signature or the secret), so a link that "does not work" can be diagnosed from the Lambda logs.
fn checked(
    app: &App,
    req: &Request,
    step: &str,
    action: &str,
    token: &str,
    sig: &str,
) -> Option<(String, String)> {
    let decoded = decode_token(token);
    let (pk, sk) = decoded.clone().unwrap_or_default();
    let shape_ok = action == "delete" && pk.starts_with("P#") && sk.starts_with("C#");
    if shape_ok && verify(&app.secret, &[action, &pk, &sk], sig) {
        return Some((pk, sk));
    }
    let agent = req
        .headers()
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("-");
    eprintln!(
        "moderation link rejected: step={step} action={action:?} token_ok={} shape_ok={shape_ok} token_len={} sig_len={} \
         sig_is_hex={} user_agent={agent:?}",
        decoded.is_some(),
        token.len(),
        sig.len(),
        sig.chars().all(|c| c.is_ascii_hexdigit())
    );
    None
}

fn page(title: &str, inner: &str) -> Result<Response<Body>, Error> {
    html(format!(
        "<!doctype html><html lang=en><meta charset=utf-8><meta name=robots content=noindex>\
         <meta name=viewport content=\"width=device-width,initial-scale=1\"><title>{t}</title>\
         <style>body{{font:16px/1.5 system-ui;max-width:40rem;margin:2rem auto;padding:0 1rem}}\
         blockquote{{border-left:4px solid #999;margin:1rem 0;padding:.25rem 1rem;white-space:pre-wrap}}\
         button{{font:inherit;padding:.5rem 1rem}}</style><h1>{t}</h1>{inner}",
        t = html_escape(title)
    ))
}

pub async fn show(app: &App, req: &Request) -> Result<Response<Body>, Error> {
    let q = req.query_string_parameters();
    let (action, token, sig) = (
        q.first("a").unwrap_or(""),
        q.first("t").unwrap_or(""),
        q.first("s").unwrap_or(""),
    );
    let Some((pk, sk)) = checked(app, req, "open", action, token, sig) else {
        return json(403, json!({ "error": "invalid link" }));
    };
    let Some(comment) = app.store.get_comment(&pk, &sk).await? else {
        return page("Comment not found", "<p>It was already deleted.</p>");
    };
    let inner = format!(
        "<p>Post: <b>{}</b><br>From: <b>{}</b><br>Status: {}</p><blockquote>{}</blockquote>\
         <form method=post action=\"/moderate\">\
         <input type=hidden name=a value=\"{}\"><input type=hidden name=t value=\"{}\">\
         <input type=hidden name=s value=\"{}\"><button>Delete for good</button></form>",
        html_escape(pk.trim_start_matches("P#")),
        html_escape(&comment.name),
        html_escape(&comment.status),
        html_escape(&comment.body),
        html_escape(action),
        html_escape(token),
        html_escape(sig),
    );
    page("Delete this comment?", &inner)
}

pub async fn apply(app: &App, req: &Request) -> Result<Response<Body>, Error> {
    if body_bytes(req).len() > MAX_BODY_BYTES {
        return json(413, json!({ "error": "request too large" }));
    }
    let Ok(form) = serde_urlencoded::from_bytes::<Form>(body_bytes(req)) else {
        return json(400, json!({ "error": "invalid request" }));
    };
    let Some((pk, sk)) = checked(app, req, "confirm", &form.a, &form.t, &form.s) else {
        return json(403, json!({ "error": "invalid link" }));
    };
    app.store.delete_comment(&pk, &sk).await?;
    page("Deleted", "<p>The comment was removed.</p>")
}
