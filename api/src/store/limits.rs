use super::reactions::is_conditional_failure;
use super::{Res, Store};
use aws_sdk_dynamodb::types::AttributeValue as A;

impl Store {
    /// Counts one hit for `who` in `bucket`; false when that is over `max` hits in the current window of
    /// `window_s` seconds. Each window has its own item, so an expired item that TTL has not deleted yet never
    /// blocks anyone. Use a fixed `who` (like "all") for an overall cap.
    pub async fn allow(
        &self,
        bucket: &str,
        who: &str,
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
                A::S(format!("RL#{bucket}#{who}#{}", now_s / window_s)),
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
            Err(e) if is_conditional_failure(&e) => Ok(false),
            Err(e) => Err(e.into()),
        }
    }
}
