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
    /// Stores a new comment of a post and returns its (PK, SK, id). A published ("approved") comment is visible at
    /// once; a held ("pending") one stays hidden until someone changes its status, and is deleted after
    /// PENDING_TTL_DAYS if nobody does.
    pub async fn add_comment(
        &self,
        page: &str,
        name: &str,
        body: &str,
        lang: &str,
        published: bool,
        now_ms: u64,
    ) -> Res<(String, String, String)> {
        let id = uuid::Uuid::new_v4().simple().to_string();
        let pk = format!("P#{page}");
        let sk = format!("C#{now_ms:013}#{id}");
        let mut put = self
            .client
            .put_item()
            .table_name(&self.table)
            .item("PK", A::S(pk.clone()))
            .item("SK", A::S(sk.clone()))
            .item("id", A::S(id.clone()))
            .item("name", A::S(name.to_string()))
            .item("body", A::S(body.to_string()))
            .item("lang", A::S(lang.to_string()))
            .item(
                "status",
                A::S(if published { "approved" } else { "pending" }.into()),
            )
            .item("created", A::N(now_ms.to_string()));
        if !published {
            let expires = now_ms / 1000 + PENDING_TTL_DAYS * 86_400;
            put = put.item("ttl", A::N(expires.to_string()));
        }
        put.send().await?;
        Ok((pk, sk, id))
    }

    /// Published comments of a post, oldest first.
    pub async fn approved_comments(&self, page: &str) -> Res<Vec<PublicComment>> {
        let out = self
            .client
            .query()
            .table_name(&self.table)
            .key_condition_expression("PK = :p AND begins_with(SK, :c)")
            .filter_expression("#s = :a")
            .expression_attribute_names("#s", "status")
            .expression_attribute_values(":p", A::S(format!("P#{page}")))
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
