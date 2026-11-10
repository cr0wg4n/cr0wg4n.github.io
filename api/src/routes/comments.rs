use crate::config::*;
use crate::http::{api_domain, body_bytes, client_ip, json, now_ms, now_s, path_param};
use crate::notify;
use crate::security::{clean_text, ip_hash, normalize_path};
use lambda_http::{Body, Error, Request, Response};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct NewComment {
    path: String,
    name: String,
    body: String,
    /// Honeypot: real visitors never see this field, bots fill it.
    #[serde(default)]
    website: String,
    /// When the form was shown (ms since epoch).
    #[serde(default)]
    ts: u64,
}

pub async fn list(app: &App, req: &Request) -> Result<Response<Body>, Error> {
    let Some(path) = path_param(req) else {
        return json(400, json!({ "error": "invalid path" }));
    };
    json(
        200,
        json!({ "comments": app.store.approved_comments(&path).await? }),
    )
}

pub async fn create(app: &App, req: &Request) -> Result<Response<Body>, Error> {
    if body_bytes(req).len() > MAX_BODY_BYTES {
        return json(413, json!({ "error": "request too large" }));
    }
    let Ok(input) = serde_json::from_slice::<NewComment>(body_bytes(req)) else {
        return json(400, json!({ "error": "invalid request" }));
    };
    let (Some(path), Some(name), Some(body)) = (
        normalize_path(&input.path),
        clean_text(&input.name, MAX_NAME_CHARS, false),
        clean_text(&input.body, MAX_COMMENT_CHARS, true),
    ) else {
        return json(400, json!({ "error": "invalid fields" }));
    };

    // Bots get the same answer as people, but nothing is stored.
    let too_fast = input.ts == 0 || now_ms().saturating_sub(input.ts) < MIN_FILL_TIME_MS;
    if !input.website.is_empty() || too_fast {
        return json(202, json!({ "status": "pending" }));
    }

    let now = now_s();
    let ip = ip_hash(&app.secret, &client_ip(req), now / 86_400);
    if !app
        .store
        .allow("comment-ip", &ip, COMMENTS_PER_IP_PER_HOUR, 3600, now)
        .await?
    {
        return json(
            429,
            json!({ "error": "too many comments from your connection, try again later" }),
        );
    }
    if !app
        .store
        .allow("comment", "all", COMMENTS_PER_HOUR_OVERALL, 3600, now)
        .await?
    {
        return json(
            429,
            json!({ "error": "the site is receiving too many comments right now, try again later" }),
        );
    }

    let (pk, sk) = app.store.add_comment(&path, &name, &body, now_ms()).await?;

    // A failed e-mail must not lose the comment: it stays pending in the table.
    if let Err(e) = notify::new_comment(app, &api_domain(req), &path, &name, &body, &pk, &sk).await
    {
        eprintln!("could not send the moderation e-mail: {e}");
    }
    json(202, json!({ "status": "pending" }))
}
