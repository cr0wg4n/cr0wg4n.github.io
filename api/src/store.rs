//! DynamoDB access. One table, keys PK/SK, TTL attribute "ttl".
//!
//!   PK = P#<path>        SK = C#<ms, 13 digits>#<id>   a comment   (id, name, body, status, created)
//!   PK = P#<path>        SK = R#<kind>                 a reaction counter (count)
//!   PK = RX#<visitor>#<path>#<kind>  SK = x            "this visitor already reacted" marker (ttl)
//!   PK = RL#<bucket>#<visitor>       SK = x            rate-limit counter (n, ttl)
//!
//! "name", "status", "count" and "ttl" are DynamoDB reserved words, hence the #placeholders.

use aws_sdk_dynamodb::types::AttributeValue as A;
use aws_sdk_dynamodb::Client;
use serde::Serialize;
use std::collections::BTreeMap;

pub type Res<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub const REACTION_KINDS: [&str; 4] = ["like", "love", "idea", "party"];

pub struct Store {
    client: Client,
    table: String,
}

#[derive(Serialize)]
pub struct PublicComment {
    pub id: String,
    pub name: String,
    pub body: String,
    pub created: u64,
}

pub struct StoredComment {
    pub name: String,
    pub body: String,
    pub status: String,
}

fn s(item: &std::collections::HashMap<String, A>, key: &str) -> String {
    item.get(key)
        .and_then(|v| v.as_s().ok())
        .cloned()
        .unwrap_or_default()
}

fn n(item: &std::collections::HashMap<String, A>, key: &str) -> u64 {
    item.get(key)
        .and_then(|v| v.as_n().ok())
        .and_then(|v| v.parse().ok())
        .unwrap_or(0)
}

impl Store {
    pub fn new(client: Client, table: String) -> Self {
        Self { client, table }
    }

    /// Stores a new comment as "pending" and returns its (PK, SK).
    pub async fn add_comment(
        &self,
        path: &str,
        name: &str,
        body: &str,
        now_ms: u64,
    ) -> Res<(String, String)> {
        let id = uuid::Uuid::new_v4().simple().to_string();
        let pk = format!("P#{path}");
        let sk = format!("C#{now_ms:013}#{id}");
        self.client
            .put_item()
            .table_name(&self.table)
            .item("PK", A::S(pk.clone()))
            .item("SK", A::S(sk.clone()))
            .item("id", A::S(id))
            .item("name", A::S(name.to_string()))
            .item("body", A::S(body.to_string()))
            .item("status", A::S("pending".into()))
            .item("created", A::N(now_ms.to_string()))
            .send()
            .await?;
        Ok((pk, sk))
    }

    /// Approved comments of a page, oldest first.
    pub async fn approved_comments(&self, path: &str) -> Res<Vec<PublicComment>> {
        let out = self
            .client
            .query()
            .table_name(&self.table)
            .key_condition_expression("PK = :p AND begins_with(SK, :c)")
            .filter_expression("#s = :a")
            .expression_attribute_names("#s", "status")
            .expression_attribute_values(":p", A::S(format!("P#{path}")))
            .expression_attribute_values(":c", A::S("C#".into()))
            .expression_attribute_values(":a", A::S("approved".into()))
            .limit(500)
            .send()
            .await?;
        Ok(out
            .items()
            .iter()
            .map(|it| PublicComment {
                id: s(it, "id"),
                name: s(it, "name"),
                body: s(it, "body"),
                created: n(it, "created"),
            })
            .collect())
    }

    pub async fn get_comment(&self, pk: &str, sk: &str) -> Res<Option<StoredComment>> {
        let out = self
            .client
            .get_item()
            .table_name(&self.table)
            .key("PK", A::S(pk.into()))
            .key("SK", A::S(sk.into()))
            .send()
            .await?;
        Ok(out.item().map(|it| StoredComment {
            name: s(it, "name"),
            body: s(it, "body"),
            status: s(it, "status"),
        }))
    }

    pub async fn approve(&self, pk: &str, sk: &str) -> Res<()> {
        self.client
            .update_item()
            .table_name(&self.table)
            .key("PK", A::S(pk.into()))
            .key("SK", A::S(sk.into()))
            .update_expression("SET #s = :a")
            .condition_expression("attribute_exists(PK)")
            .expression_attribute_names("#s", "status")
            .expression_attribute_values(":a", A::S("approved".into()))
            .send()
            .await?;
        Ok(())
    }

    pub async fn delete_comment(&self, pk: &str, sk: &str) -> Res<()> {
        self.client
            .delete_item()
            .table_name(&self.table)
            .key("PK", A::S(pk.into()))
            .key("SK", A::S(sk.into()))
            .send()
            .await?;
        Ok(())
    }

    /// Reaction counters of a page; every known kind is present (0 when never used).
    pub async fn reactions(&self, path: &str) -> Res<BTreeMap<String, u64>> {
        let mut counts: BTreeMap<String, u64> =
            REACTION_KINDS.iter().map(|k| (k.to_string(), 0)).collect();
        let out = self
            .client
            .query()
            .table_name(&self.table)
            .key_condition_expression("PK = :p AND begins_with(SK, :r)")
            .expression_attribute_values(":p", A::S(format!("P#{path}")))
            .expression_attribute_values(":r", A::S("R#".into()))
            .send()
            .await?;
        for it in out.items() {
            let kind = s(it, "SK").trim_start_matches("R#").to_string();
            if let Some(slot) = counts.get_mut(&kind) {
                *slot = n(it, "count");
            }
        }
        Ok(counts)
    }

    /// Adds one reaction unless this visitor already used this kind on this page. Returns whether it was added.
    pub async fn add_reaction(
        &self,
        path: &str,
        kind: &str,
        visitor: &str,
        now_s: u64,
    ) -> Res<bool> {
        let marker = self
            .client
            .put_item()
            .table_name(&self.table)
            .item("PK", A::S(format!("RX#{visitor}#{path}#{kind}")))
            .item("SK", A::S("x".into()))
            .item("ttl", A::N((now_s + 2 * 86_400).to_string()))
            .condition_expression("attribute_not_exists(PK)")
            .send()
            .await;
        match marker {
            Ok(_) => {}
            Err(e)
                if e.as_service_error()
                    .is_some_and(|se| se.is_conditional_check_failed_exception()) =>
            {
                return Ok(false)
            }
            Err(e) => return Err(e.into()),
        }
        self.client
            .update_item()
            .table_name(&self.table)
            .key("PK", A::S(format!("P#{path}")))
            .key("SK", A::S(format!("R#{kind}")))
            .update_expression("ADD #c :one")
            .expression_attribute_names("#c", "count")
            .expression_attribute_values(":one", A::N("1".into()))
            .send()
            .await?;
        Ok(true)
    }

    /// Counts one hit in a bucket; false when the visitor is over `max` hits in the current window of `window_s`
    /// seconds. Each window has its own item, so an expired item that TTL has not deleted yet never blocks anyone.
    pub async fn allow(
        &self,
        bucket: &str,
        visitor: &str,
        max: u64,
        window_s: u64,
        now_s: u64,
    ) -> Res<bool> {
        let res = self
            .client
            .update_item()
            .table_name(&self.table)
            .key(
                "PK",
                A::S(format!("RL#{bucket}#{visitor}#{}", now_s / window_s)),
            )
            .key("SK", A::S("x".into()))
            .update_expression("SET #t = :ttl ADD #n :one")
            .condition_expression("attribute_not_exists(#n) OR #n < :max")
            .expression_attribute_names("#t", "ttl")
            .expression_attribute_names("#n", "n")
            .expression_attribute_values(":ttl", A::N((now_s + 2 * window_s).to_string()))
            .expression_attribute_values(":one", A::N("1".into()))
            .expression_attribute_values(":max", A::N(max.to_string()))
            .send()
            .await;
        match res {
            Ok(_) => Ok(true),
            Err(e)
                if e.as_service_error()
                    .is_some_and(|se| se.is_conditional_check_failed_exception()) =>
            {
                Ok(false)
            }
            Err(e) => Err(e.into()),
        }
    }
}
