//! The delete link from the new-comment e-mail. A link only opens a confirmation page (mail scanners open links);
//! the change happens when the button on that page is pressed.

use crate::config::{App, MAX_BODY_BYTES};
use crate::http::{body_bytes, html, json};
use crate::security::{html_escape, verify};
use lambda_http::{Body, Error, Request, RequestExt, Response};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct Form {
    action: String,
    p: String,
    k: String,
    sig: String,
}

fn valid(app: &App, action: &str, pk: &str, sk: &str, sig: &str) -> bool {
    action == "delete"
        && pk.starts_with("P#")
        && sk.starts_with("C#")
        && verify(&app.secret, &[action, pk, sk], sig)
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
    let (action, pk, sk, sig) = (
        q.first("action").unwrap_or(""),
        q.first("p").unwrap_or(""),
        q.first("k").unwrap_or(""),
        q.first("sig").unwrap_or(""),
    );
    if !valid(app, action, pk, sk, sig) {
        return json(403, json!({ "error": "invalid link" }));
    }
    let Some(comment) = app.store.get_comment(pk, sk).await? else {
        return page("Comment not found", "<p>It was already deleted.</p>");
    };
    let inner = format!(
        "<p>Page: <b>{}</b><br>From: <b>{}</b><br>Status: {}</p><blockquote>{}</blockquote>\
         <form method=post action=\"/moderate\">\
         <input type=hidden name=action value=\"{}\"><input type=hidden name=p value=\"{}\">\
         <input type=hidden name=k value=\"{}\"><input type=hidden name=sig value=\"{}\">\
         <button>{}</button></form>",
        html_escape(pk.trim_start_matches("P#")),
        html_escape(&comment.name),
        html_escape(&comment.status),
        html_escape(&comment.body),
        html_escape(action),
        html_escape(pk),
        html_escape(sk),
        html_escape(sig),
        "Delete for good",
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
    if !valid(app, &form.action, &form.p, &form.k, &form.sig) {
        return json(403, json!({ "error": "invalid link" }));
    }
    app.store.delete_comment(&form.p, &form.k).await?;
    page("Deleted", "<p>The comment was removed.</p>")
}
