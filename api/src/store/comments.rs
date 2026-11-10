use super::{number, text, Res, Store};
use crate::config::PENDING_TTL_DAYS;
use aws_sdk_dynamodb::types::AttributeValue as A;
use serde::Serialize;

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

impl Store {
    /// Stores a new comment as "pending" (deleted after PENDING_TTL_DAYS if nobody reviews it) and returns (PK, SK).
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
        let expires = now_ms / 1000 + PENDING_TTL_DAYS * 86_400;
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
            .item("ttl", A::N(expires.to_string()))
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
                id: text(it, "id"),
                name: text(it, "name"),
                body: text(it, "body"),
                created: number(it, "created"),
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
            name: text(it, "name"),
            body: text(it, "body"),
            status: text(it, "status"),
        }))
    }

    /// Publishes a comment and removes its expiry. Fails if the comment no longer exists.
    pub async fn approve(&self, pk: &str, sk: &str) -> Res<()> {
        self.client
            .update_item()
            .table_name(&self.table)
            .key("PK", A::S(pk.into()))
            .key("SK", A::S(sk.into()))
            .update_expression("SET #s = :a REMOVE #t")
            .condition_expression("attribute_exists(PK)")
            .expression_attribute_names("#s", "status")
            .expression_attribute_names("#t", "ttl")
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
}
