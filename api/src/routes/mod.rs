//! One handler per endpoint:
//!
//!   GET  /comments?key=/posts/x     published comments of a post (all its translations)
//!   POST /comments                  new comment: published at once, the owner gets an e-mail with a delete link
//!   GET  /reactions?key=...         counters
//!   POST /reactions                 one reaction per browser, kind and post per day
//!   GET  /moderate?...              confirmation page from the e-mail link (never changes anything)
//!   POST /moderate                  delete a comment, only with a valid signature
//!
//! CORS is handled by API Gateway (see template.yaml), so no CORS headers here.

mod comments;
mod moderation;
mod reactions;

use crate::config::App;
use crate::http::json;
use lambda_http::{Body, Error, Request, Response};
use serde_json::json;
use std::sync::Arc;

pub async fn handle(app: Arc<App>, req: Request) -> Result<Response<Body>, Error> {
    let route = req.uri().path().trim_end_matches('/').to_string();
    match (req.method().as_str(), route.as_str()) {
        ("GET", "/comments") => comments::list(&app, &req).await,
        ("POST", "/comments") => comments::create(&app, &req).await,
        ("GET", "/reactions") => reactions::list(&app, &req).await,
        ("POST", "/reactions") => reactions::create(&app, &req).await,
        ("GET", "/moderate") => moderation::show(&app, &req).await,
        ("POST", "/moderate") => moderation::apply(&app, &req).await,
        _ => json(404, json!({ "error": "not found" })),
    }
}
