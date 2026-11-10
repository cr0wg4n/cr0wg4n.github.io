//! The "comment pending" e-mail, sent through an SNS topic that has the owner's address subscribed.

use crate::config::{App, NOTIFICATIONS_PER_HOUR};
use crate::http::now_s;
use crate::security::sign;
use lambda_http::Error;

/// A new comment is waiting. Past NOTIFICATIONS_PER_HOUR e-mails an hour the comment is still stored, and a single
/// summary e-mail says so, so a flood of comments cannot flood the inbox.
pub async fn new_comment(
    app: &App,
    domain: &str,
    path: &str,
    name: &str,
    body: &str,
    pk: &str,
    sk: &str,
) -> Result<(), Error> {
    let now = now_s();
    if app
        .store
        .allow("notify", "all", NOTIFICATIONS_PER_HOUR, 3600, now)
        .await?
    {
        let text = format!(
            "New comment on https://cr0wg4n.com{path}\n\nFrom: {name}\n\n{body}\n\nApprove: {}\nDelete:  {}\n",
            link(app, domain, "approve", pk, sk),
            link(app, domain, "delete", pk, sk)
        );
        publish(app, &format!("Comment pending: {path}"), text).await
    } else if app
        .store
        .allow("notify-summary", "all", 1, 3600, now)
        .await?
    {
        let text = format!(
            "More than {NOTIFICATIONS_PER_HOUR} comments arrived this hour, so the rest were stored without an e-mail each.\n\
             Review them in the DynamoDB console (items with status = pending). They are deleted after a while if nobody reviews them.\n"
        );
        publish(app, "Many comments pending", text).await
    } else {
        Ok(())
    }
}

/// A moderation link, signed so it cannot be forged or edited.
fn link(app: &App, domain: &str, action: &str, pk: &str, sk: &str) -> String {
    let sig = sign(&app.secret, &[action, pk, sk]);
    let query = serde_urlencoded::to_string([
        ("action", action),
        ("p", pk),
        ("k", sk),
        ("sig", sig.as_str()),
    ])
    .unwrap_or_default();
    format!("https://{domain}/moderate?{query}")
}

async fn publish(app: &App, subject: &str, message: String) -> Result<(), Error> {
    // SNS subjects: plain ASCII, at most 100 characters (paths only hold [A-Za-z0-9/_-]).
    let subject: String = subject.chars().take(100).collect();
    app.sns
        .publish()
        .topic_arn(&app.topic_arn)
        .subject(subject)
        .message(message)
        .send()
        .await?;
    Ok(())
}
