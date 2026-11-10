//! E-mails to the owner, sent through an SNS topic that has the owner's address subscribed.
//!
//! Comments are published at once. Each one is announced with a link to delete it. To stop a flood from filling the
//! inbox (and from publishing unseen spam), there are NOTIFICATIONS_PER_HOUR announcements an hour: past that, new
//! comments are held for review instead of published, and one summary e-mail says so.

use crate::config::{App, NOTIFICATIONS_PER_HOUR};
use crate::http::now_s;
use crate::security::sign;
use lambda_http::Error;

/// Takes one of the hourly announcement slots. False means "too many comments this hour: hold this one".
pub async fn reserve_slot(app: &App) -> Result<bool, Error> {
    app.store
        .allow("notify", "all", NOTIFICATIONS_PER_HOUR, 3600, now_s())
        .await
}

/// What the owner is told about a published comment.
pub struct Published<'a> {
    /// The API's address, to build the delete link.
    pub domain: &'a str,
    pub page: &'a str,
    pub lang: &'a str,
    pub name: &'a str,
    pub body: &'a str,
    pub pk: &'a str,
    pub sk: &'a str,
}

/// A comment was published: tell the owner, with a link to delete it.
pub async fn comment_published(app: &App, c: &Published<'_>) -> Result<(), Error> {
    let language = if c.lang == "es" { "Spanish" } else { "English" };
    let text = format!(
        "New comment published on https://cr0wg4n.com/{}/ (written on the {language} page)\n\nFrom: {}\n\n{}\n\nDelete it: {}\n",
        c.page,
        c.name,
        c.body,
        delete_link(app, c.domain, c.pk, c.sk)
    );
    publish(app, &format!("New comment: {}", c.page), text).await
}

/// Comments are being held because of the hourly cap: one summary e-mail per hour.
pub async fn comments_held(app: &App) -> Result<(), Error> {
    if !app
        .store
        .allow("notify-summary", "all", 1, 3600, now_s())
        .await?
    {
        return Ok(());
    }
    let text = format!(
        "More than {NOTIFICATIONS_PER_HOUR} comments arrived this hour, so new ones are being HELD instead of published.\n\
         Review them in the DynamoDB console: items with status = pending. To publish one, change its status to approved \
         and remove its ttl; otherwise it is deleted after a while.\n"
    );
    publish(app, "Comments are being held", text).await
}

/// A link that deletes one comment, signed so it cannot be forged or edited.
fn delete_link(app: &App, domain: &str, pk: &str, sk: &str) -> String {
    let sig = sign(&app.secret, &["delete", pk, sk]);
    let query = serde_urlencoded::to_string([
        ("action", "delete"),
        ("p", pk),
        ("k", sk),
        ("sig", sig.as_str()),
    ])
    .unwrap_or_default();
    format!("https://{domain}/moderate?{query}")
}

async fn publish(app: &App, subject: &str, message: String) -> Result<(), Error> {
    // SNS subjects: plain ASCII, at most 100 characters (pages only hold [A-Za-z0-9/_-]).
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
