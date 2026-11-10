use crate::config::*;
use crate::http::{api_domain, body_bytes, client_ip, json, now_ms, now_s, page_param};
use crate::notify;
use crate::security::{clean_text, ip_hash, normalize_page};
use lambda_http::{Body, Error, Request, Response};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct NewComment {
    /// The post: Hugo's translation key ("/posts/<slug>"), shared by its English and Spanish pages.
    /// Older pages sent their address as `path`, which is still understood.
    #[serde(alias = "path")]
    key: String,
    name: String,
    body: String,
    /// The language of the page the comment was written on ("en" or "es"); only used in the e-mail.
    #[serde(default)]
    lang: String,
    /// Honeypot: real visitors never see this field, bots fill it.
    #[serde(default)]
    website: String,
    /// When the form was shown (ms since epoch).
    #[serde(default)]
    ts: u64,
}

pub async fn list(app: &App, req: &Request) -> Result<Response<Body>, Error> {
    let Some(page) = page_param(req) else {
        return json(400, json!({ "error": "invalid page" }));
    };
    json(
        200,
        json!({ "comments": app.store.approved_comments(&page).await? }),
    )
}

pub async fn create(app: &App, req: &Request) -> Result<Response<Body>, Error> {
    if body_bytes(req).len() > MAX_BODY_BYTES {
        return json(413, json!({ "error": "request too large" }));
    }
    let Ok(input) = serde_json::from_slice::<NewComment>(body_bytes(req)) else {
        return json(400, json!({ "error": "invalid request" }));
    };
    let (Some(page), Some(name), Some(body)) = (
        normalize_page(&input.key),
        clean_text(&input.name, MAX_NAME_CHARS, false),
        clean_text(&input.body, MAX_COMMENT_CHARS, true),
    ) else {
        return json(400, json!({ "error": "invalid fields" }));
    };
    let lang = if input.lang == "es" { "es" } else { "en" };
    let now = now_ms();

    // Bots get the same answer as people (the comment "published"), but nothing is stored.
    let too_fast = input.ts == 0 || now.saturating_sub(input.ts) < MIN_FILL_TIME_MS;
    if !input.website.is_empty() || too_fast {
        return json(
            201,
            json!({ "status": "published", "comment": { "id": "-", "name": name, "body": body, "created": now } }),
        );
    }

    let now_secs = now_s();
    let ip = ip_hash(&app.secret, &client_ip(req), now_secs / 86_400);
    if !app
        .store
        .allow("comment-ip", &ip, COMMENTS_PER_IP_PER_HOUR, 3600, now_secs)
        .await?
    {
        return json(
            429,
            json!({ "error": "too many comments from your connection, try again later" }),
        );
    }
    if !app
        .store
        .allow("comment", "all", COMMENTS_PER_HOUR_OVERALL, 3600, now_secs)
        .await?
    {
        return json(
            429,
            json!({ "error": "the site is receiving too many comments right now, try again later" }),
        );
    }

    // Published at once while the owner can still be told about each comment; held for review during a flood.
    let publish = notify::reserve_slot(app).await?;
    let (pk, sk, id) = app
        .store
        .add_comment(&page, &name, &body, lang, publish, now)
        .await?;

    // A failed e-mail must not lose the comment: it is already stored.
    if publish {
        if let Err(e) = notify::comment_published(
            app,
            &notify::Published {
                domain: &api_domain(req),
                page: &page,
                lang,
                name: &name,
                body: &body,
                pk: &pk,
                sk: &sk,
            },
        )
        .await
        {
            eprintln!("could not send the new-comment e-mail: {e}");
        }
        json(
            201,
            json!({ "status": "published", "comment": { "id": id, "name": name, "body": body, "created": now } }),
        )
    } else {
        if let Err(e) = notify::comments_held(app).await {
            eprintln!("could not send the held-comments e-mail: {e}");
        }
        json(202, json!({ "status": "pending" }))
    }
}
