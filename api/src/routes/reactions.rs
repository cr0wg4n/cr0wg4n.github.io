use crate::config::*;
use crate::http::{body_bytes, client_ip, json, now_s, page_param, visitor};
use crate::security::{ip_hash, normalize_page};
use crate::store::REACTION_KINDS;
use lambda_http::{Body, Error, Request, Response};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct NewReaction {
    /// The post's translation key; older pages sent their address as `path`.
    #[serde(alias = "path")]
    key: String,
    kind: String,
}

pub async fn list(app: &App, req: &Request) -> Result<Response<Body>, Error> {
    let Some(page) = page_param(req) else {
        return json(400, json!({ "error": "invalid page" }));
    };
    json(
        200,
        json!({ "reactions": app.store.reactions(&page).await? }),
    )
}

pub async fn create(app: &App, req: &Request) -> Result<Response<Body>, Error> {
    if body_bytes(req).len() > MAX_BODY_BYTES {
        return json(413, json!({ "error": "request too large" }));
    }
    let Ok(input) = serde_json::from_slice::<NewReaction>(body_bytes(req)) else {
        return json(400, json!({ "error": "invalid request" }));
    };
    let Some(page) = normalize_page(&input.key) else {
        return json(400, json!({ "error": "invalid page" }));
    };
    if !REACTION_KINDS.contains(&input.kind.as_str()) {
        return json(400, json!({ "error": "invalid reaction" }));
    }

    let now = now_s();
    // The volume limit is per address; "already reacted" is per browser, so people sharing an address still count.
    let ip = ip_hash(&app.secret, &client_ip(req), now / 86_400);
    if !app
        .store
        .allow("reaction-ip", &ip, REACTIONS_PER_IP_PER_HOUR, 3600, now)
        .await?
    {
        return json(
            429,
            json!({ "error": "too many reactions, try again later" }),
        );
    }
    let added = app
        .store
        .add_reaction(&page, &input.kind, &visitor(&app.secret, req), now)
        .await?;
    json(
        200,
        json!({ "added": added, "reactions": app.store.reactions(&page).await? }),
    )
}
