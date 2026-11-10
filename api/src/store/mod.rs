//! DynamoDB access. One table, keys PK/SK, TTL attribute "ttl".
//!
//!   PK = P#<path>        SK = C#<ms, 13 digits>#<id>   a comment   (id, name, body, status, created, ttl while pending)
//!   PK = P#<path>        SK = R#<kind>                 a reaction counter (count)
//!   PK = RX#<visitor>#<path>#<kind>  SK = x            "this browser already reacted" marker (ttl)
//!   PK = RL#<bucket>#<who>#<window>  SK = x            rate-limit counter (n, ttl)
//!
//! "name", "status", "count" and "ttl" are DynamoDB reserved words, hence the #placeholders in expressions.

mod comments;
mod limits;
mod reactions;

pub use comments::{PublicComment, StoredComment};
pub use reactions::REACTION_KINDS;

use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;

pub type Res<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct Store {
    client: Client,
    table: String,
}

impl Store {
    pub fn new(client: Client, table: String) -> Self {
        Self { client, table }
    }
}

type Item = HashMap<String, AttributeValue>;

fn text(item: &Item, key: &str) -> String {
    item.get(key)
        .and_then(|v| v.as_s().ok())
        .cloned()
        .unwrap_or_default()
}

fn number(item: &Item, key: &str) -> u64 {
    item.get(key)
        .and_then(|v| v.as_n().ok())
        .and_then(|v| v.parse().ok())
        .unwrap_or(0)
}
