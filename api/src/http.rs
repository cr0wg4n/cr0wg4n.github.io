//! Reading requests and building responses.

use crate::security::{normalize_page, visitor_hash};
use lambda_http::request::RequestContext;
use lambda_http::{Body, Error, Request, RequestExt, Response};
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

pub fn now_s() -> u64 {
    now_ms() / 1000
}

fn build(status: u16, content_type: &str, body: String) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(status)
        .header("content-type", content_type)
        .header("cache-control", "no-store")
        .header("x-content-type-options", "nosniff")
        .body(Body::from(body))?)
}

pub fn json(status: u16, value: Value) -> Result<Response<Body>, Error> {
    build(status, "application/json; charset=utf-8", value.to_string())
}

/// An HTML page for the moderation links. It loads nothing and may only post back to this API.
pub fn html(body: String) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html; charset=utf-8")
        .header("cache-control", "no-store")
        .header(
            "content-security-policy",
            "default-src 'none'; style-src 'unsafe-inline'; form-action 'self'",
        )
        .body(Body::from(body))?)
}

pub fn body_bytes(req: &Request) -> &[u8] {
    match req.body() {
        Body::Text(t) => t.as_bytes(),
        Body::Binary(b) => b,
        _ => &[],
    }
}

fn gateway_context(
    req: &Request,
) -> Option<lambda_http::aws_lambda_events::apigw::ApiGatewayV2httpRequestContext> {
    match req.request_context() {
        RequestContext::ApiGatewayV2(ctx) => Some(ctx),
        _ => None,
    }
}

pub fn client_ip(req: &Request) -> String {
    gateway_context(req)
        .and_then(|ctx| ctx.http.source_ip)
        .unwrap_or_default()
}

/// The host the request came in through, used to build the moderation links.
pub fn api_domain(req: &Request) -> String {
    gateway_context(req)
        .and_then(|ctx| ctx.domain_name)
        .unwrap_or_default()
}

/// One-way daily identifier of this browser (address + user agent): for "already reacted" markers.
pub fn visitor(secret: &[u8], req: &Request) -> String {
    let ua = req
        .headers()
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    visitor_hash(secret, &client_ip(req), ua, now_s() / 86_400)
}

/// The page a request is about: the `key` query parameter (or the older `path`), as "posts/<slug>".
pub fn page_param(req: &Request) -> Option<String> {
    let query = req.query_string_parameters();
    query
        .first("key")
        .or_else(|| query.first("path"))
        .and_then(normalize_page)
}
