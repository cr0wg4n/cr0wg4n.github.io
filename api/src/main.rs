//! Comments and reactions API for cr0wg4n.com.
//!
//!   GET  /comments?path=/posts/x/   approved comments
//!   POST /comments                  new comment, stored as "pending"; the owner gets an e-mail
//!   GET  /reactions?path=...        counters
//!   POST /reactions                 one reaction per visitor, kind and page per day
//!   GET  /moderate?...              confirmation page from the e-mail link (never changes anything)
//!   POST /moderate                  approve or delete, only with a valid signature
//!
//! CORS is handled by API Gateway (see template.yaml), so no CORS headers here.

use comments_api::security::{clean_text, html_escape, normalize_path, sign, verify, visitor_hash};
use comments_api::store::{Store, REACTION_KINDS};
use lambda_http::request::RequestContext;
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

struct App {
    store: Store,
    sns: aws_sdk_sns::Client,
    topic_arn: String,
    secret: Vec<u8>,
}

#[derive(Deserialize)]
struct NewComment {
    path: String,
    name: String,
    body: String,
    /// Honeypot: real visitors never see this field, bots fill it.
    #[serde(default)]
    website: String,
    /// When the form was shown (ms since epoch). A form "sent" within 3 s is a bot.
    #[serde(default)]
    ts: u64,
}

#[derive(Deserialize)]
struct NewReaction {
    path: String,
    kind: String,
}

#[derive(Deserialize)]
struct ModerationForm {
    action: String,
    p: String,
    k: String,
    sig: String,
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn respond(status: u16, content_type: &str, body: String) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(status)
        .header("content-type", content_type)
        .header("cache-control", "no-store")
        .header("x-content-type-options", "nosniff")
        .body(Body::from(body))?)
}

fn json_response(status: u16, value: Value) -> Result<Response<Body>, Error> {
    respond(status, "application/json; charset=utf-8", value.to_string())
}

fn body_bytes(req: &Request) -> &[u8] {
    match req.body() {
        Body::Text(t) => t.as_bytes(),
        Body::Binary(b) => b,
        _ => &[],
    }
}

fn client_ip(req: &Request) -> String {
    match req.request_context() {
        RequestContext::ApiGatewayV2(ctx) => ctx.http.source_ip.unwrap_or_default(),
        _ => String::new(),
    }
}

fn api_domain(req: &Request) -> String {
    match req.request_context() {
        RequestContext::ApiGatewayV2(ctx) => ctx.domain_name.unwrap_or_default(),
        _ => String::new(),
    }
}

fn visitor(app: &App, req: &Request, now_s: u64) -> String {
    let ua = req
        .headers()
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    visitor_hash(&app.secret, &client_ip(req), ua, now_s / 86_400)
}

async fn handle(app: Arc<App>, req: Request) -> Result<Response<Body>, Error> {
    let method = req.method().as_str().to_string();
    let route = req.uri().path().trim_end_matches('/').to_string();
    match (method.as_str(), route.as_str()) {
        ("GET", "/comments") => list_comments(&app, &req).await,
        ("POST", "/comments") => post_comment(&app, &req).await,
        ("GET", "/reactions") => get_reactions(&app, &req).await,
        ("POST", "/reactions") => post_reaction(&app, &req).await,
        ("GET", "/moderate") => moderation_page(&app, &req).await,
        ("POST", "/moderate") => moderation_apply(&app, &req).await,
        _ => json_response(404, json!({ "error": "not found" })),
    }
}

fn path_param(req: &Request) -> Option<String> {
    req.query_string_parameters()
        .first("path")
        .and_then(normalize_path)
}

async fn list_comments(app: &App, req: &Request) -> Result<Response<Body>, Error> {
    let Some(path) = path_param(req) else {
        return json_response(400, json!({ "error": "invalid path" }));
    };
    let comments = app.store.approved_comments(&path).await?;
    json_response(200, json!({ "comments": comments }))
}

async fn post_comment(app: &App, req: &Request) -> Result<Response<Body>, Error> {
    let Ok(input) = serde_json::from_slice::<NewComment>(body_bytes(req)) else {
        return json_response(400, json!({ "error": "invalid request" }));
    };
    let (Some(path), Some(name), Some(body)) = (
        normalize_path(&input.path),
        clean_text(&input.name, 60, false),
        clean_text(&input.body, 2000, true),
    ) else {
        return json_response(400, json!({ "error": "invalid fields" }));
    };

    // Bots get the same answer as people, but nothing is stored.
    let too_fast = input.ts == 0 || now_ms().saturating_sub(input.ts) < 3_000;
    if !input.website.is_empty() || too_fast {
        return json_response(202, json!({ "status": "pending" }));
    }

    let now_s = now_ms() / 1000;
    let who = visitor(app, req, now_s);
    if !app.store.allow("comment", &who, 3, 3600, now_s).await? {
        return json_response(
            429,
            json!({ "error": "too many comments, try again later" }),
        );
    }

    let (pk, sk) = app.store.add_comment(&path, &name, &body, now_ms()).await?;

    // A failed e-mail must not lose the comment: it stays pending in the table.
    if let Err(e) = notify_owner(app, &api_domain(req), &path, &name, &body, &pk, &sk).await {
        eprintln!("could not send the moderation e-mail: {e}");
    }
    json_response(202, json!({ "status": "pending" }))
}

async fn notify_owner(
    app: &App,
    domain: &str,
    path: &str,
    name: &str,
    body: &str,
    pk: &str,
    sk: &str,
) -> Result<(), Error> {
    let link = |action: &str| {
        let sig = sign(&app.secret, &[action, pk, sk]);
        format!(
            "https://{domain}/moderate?{}",
            serde_urlencoded::to_string([
                ("action", action),
                ("p", pk),
                ("k", sk),
                ("sig", sig.as_str())
            ])
            .unwrap_or_default()
        )
    };
    let text = format!(
        "New comment on https://cr0wg4n.com{path}\n\nFrom: {name}\n\n{body}\n\nApprove: {}\nDelete:  {}\n",
        link("approve"),
        link("delete")
    );
    // SNS subjects must be plain ASCII, at most 100 characters; the path only holds [A-Za-z0-9/_.-].
    let subject: String = format!("Comment pending: {path}")
        .chars()
        .take(100)
        .collect();
    app.sns
        .publish()
        .topic_arn(&app.topic_arn)
        .subject(subject)
        .message(text)
        .send()
        .await?;
    Ok(())
}

async fn get_reactions(app: &App, req: &Request) -> Result<Response<Body>, Error> {
    let Some(path) = path_param(req) else {
        return json_response(400, json!({ "error": "invalid path" }));
    };
    json_response(
        200,
        json!({ "reactions": app.store.reactions(&path).await? }),
    )
}

async fn post_reaction(app: &App, req: &Request) -> Result<Response<Body>, Error> {
    let Ok(input) = serde_json::from_slice::<NewReaction>(body_bytes(req)) else {
        return json_response(400, json!({ "error": "invalid request" }));
    };
    let Some(path) = normalize_path(&input.path) else {
        return json_response(400, json!({ "error": "invalid path" }));
    };
    if !REACTION_KINDS.contains(&input.kind.as_str()) {
        return json_response(400, json!({ "error": "invalid reaction" }));
    }
    let now_s = now_ms() / 1000;
    let who = visitor(app, req, now_s);
    if !app.store.allow("reaction", &who, 30, 3600, now_s).await? {
        return json_response(
            429,
            json!({ "error": "too many reactions, try again later" }),
        );
    }
    let added = app
        .store
        .add_reaction(&path, &input.kind, &who, now_s)
        .await?;
    json_response(
        200,
        json!({ "added": added, "reactions": app.store.reactions(&path).await? }),
    )
}

fn moderation_fields(app: &App, action: &str, pk: &str, sk: &str, sig: &str) -> bool {
    matches!(action, "approve" | "delete")
        && pk.starts_with("P#")
        && sk.starts_with("C#")
        && verify(&app.secret, &[action, pk, sk], sig)
}

fn page(title: &str, inner: &str) -> Result<Response<Body>, Error> {
    let html = format!(
        "<!doctype html><html lang=en><meta charset=utf-8><meta name=robots content=noindex>\
         <meta name=viewport content=\"width=device-width,initial-scale=1\"><title>{t}</title>\
         <style>body{{font:16px/1.5 system-ui;max-width:40rem;margin:2rem auto;padding:0 1rem}}\
         blockquote{{border-left:4px solid #999;margin:1rem 0;padding:.25rem 1rem;white-space:pre-wrap}}\
         button{{font:inherit;padding:.5rem 1rem}}</style><h1>{t}</h1>{inner}",
        t = html_escape(title)
    );
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html; charset=utf-8")
        .header("cache-control", "no-store")
        .header(
            "content-security-policy",
            "default-src 'none'; style-src 'unsafe-inline'; form-action 'self'",
        )
        .body(Body::from(html))?)
}

async fn moderation_page(app: &App, req: &Request) -> Result<Response<Body>, Error> {
    let q = req.query_string_parameters();
    let (action, pk, sk, sig) = (
        q.first("action").unwrap_or(""),
        q.first("p").unwrap_or(""),
        q.first("k").unwrap_or(""),
        q.first("sig").unwrap_or(""),
    );
    if !moderation_fields(app, action, pk, sk, sig) {
        return json_response(403, json!({ "error": "invalid link" }));
    }
    let Some(comment) = app.store.get_comment(pk, sk).await? else {
        return page("Comment not found", "<p>It was already deleted.</p>");
    };
    // The link only shows this page; the change happens when the button is pressed (mail scanners open links).
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
        if action == "approve" {
            "Approve and publish"
        } else {
            "Delete for good"
        },
    );
    page(
        if action == "approve" {
            "Approve this comment?"
        } else {
            "Delete this comment?"
        },
        &inner,
    )
}

async fn moderation_apply(app: &App, req: &Request) -> Result<Response<Body>, Error> {
    let Ok(form) = serde_urlencoded::from_bytes::<ModerationForm>(body_bytes(req)) else {
        return json_response(400, json!({ "error": "invalid request" }));
    };
    if !moderation_fields(app, &form.action, &form.p, &form.k, &form.sig) {
        return json_response(403, json!({ "error": "invalid link" }));
    }
    if form.action == "approve" {
        match app.store.approve(&form.p, &form.k).await {
            Ok(()) => page("Published", "<p>The comment is now visible.</p>"),
            Err(_) => page("Not found", "<p>The comment no longer exists.</p>"),
        }
    } else {
        app.store.delete_comment(&form.p, &form.k).await?;
        page("Deleted", "<p>The comment was removed.</p>")
    }
}

fn env(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("missing environment variable {name}"))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let ssm = aws_sdk_ssm::Client::new(&config);
    let secret = ssm
        .get_parameter()
        .name(env("SECRET_PARAM"))
        .with_decryption(true)
        .send()
        .await?
        .parameter()
        .and_then(|p| p.value().map(|v| v.as_bytes().to_vec()))
        .expect("the secret parameter has no value");
    let app = Arc::new(App {
        store: Store::new(aws_sdk_dynamodb::Client::new(&config), env("TABLE_NAME")),
        sns: aws_sdk_sns::Client::new(&config),
        topic_arn: env("TOPIC_ARN"),
        secret,
    });
    run(service_fn(move |req| handle(app.clone(), req))).await
}
