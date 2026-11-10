use super::{number, text, Res, Store};
use aws_sdk_dynamodb::error::{ProvideErrorMetadata, SdkError};
use aws_sdk_dynamodb::types::AttributeValue as A;
use std::collections::BTreeMap;

pub const REACTION_KINDS: [&str; 4] = ["like", "love", "idea", "party"];

impl Store {
    /// Reaction counters of a page; every known kind is present (0 when never used).
    pub async fn reactions(&self, page: &str) -> Res<BTreeMap<String, u64>> {
        let mut counts: BTreeMap<String, u64> =
            REACTION_KINDS.iter().map(|k| (k.to_string(), 0)).collect();
        let out = self
            .client
            .query()
            .table_name(&self.table)
            .key_condition_expression("PK = :p AND begins_with(SK, :r)")
            .expression_attribute_values(":p", A::S(format!("P#{page}")))
            .expression_attribute_values(":r", A::S("R#".into()))
            .send()
            .await?;
        for it in out.items() {
            let kind = text(it, "SK").trim_start_matches("R#").to_string();
            if let Some(slot) = counts.get_mut(&kind) {
                *slot = number(it, "count");
            }
        }
        Ok(counts)
    }

    /// Adds one reaction unless this browser already used this kind on this post today. Returns whether it was added.
    pub async fn add_reaction(
        &self,
        page: &str,
        kind: &str,
        visitor: &str,
        now_s: u64,
    ) -> Res<bool> {
        let marker = self
            .client
            .put_item()
            .table_name(&self.table)
            .item("PK", A::S(format!("RX#{visitor}#{page}#{kind}")))
            .item("SK", A::S("x".into()))
            .item("ttl", A::N((now_s + 2 * 86_400).to_string()))
            .condition_expression("attribute_not_exists(PK)")
            .send()
            .await;
        match marker {
            Ok(_) => {}
            Err(e) if is_conditional_failure(&e) => return Ok(false),
            Err(e) => return Err(e.into()),
        }
        self.client
            .update_item()
            .table_name(&self.table)
            .key("PK", A::S(format!("P#{page}")))
            .key("SK", A::S(format!("R#{kind}")))
            .update_expression("ADD #c :one")
            .expression_attribute_names("#c", "count")
            .expression_attribute_values(":one", A::N("1".into()))
            .send()
            .await?;
        Ok(true)
    }
}

pub(super) fn is_conditional_failure<E, R>(e: &SdkError<E, R>) -> bool
where
    E: ProvideErrorMetadata,
{
    e.code() == Some("ConditionalCheckFailedException")
}
