//! Environment, shared state and every tunable limit in one place.

use crate::store::Store;
use lambda_http::Error;

/// Per IP address, per hour. Generous for real readers (an office or a mobile network shares one address).
pub const COMMENTS_PER_IP_PER_HOUR: u64 = 5;
pub const REACTIONS_PER_IP_PER_HOUR: u64 = 60;
/// Across all visitors, per hour: stops a flood from many addresses.
pub const COMMENTS_PER_HOUR_OVERALL: u64 = 120;
/// "Comment pending" e-mails per hour. Comments over the cap are still stored; one summary e-mail says so.
pub const NOTIFICATIONS_PER_HOUR: u64 = 20;
/// A comment form sent sooner than this after the page loaded is a bot.
pub const MIN_FILL_TIME_MS: u64 = 3_000;
/// Requests bigger than this are rejected before parsing (a comment is at most 2000 characters, up to 4 bytes each).
pub const MAX_BODY_BYTES: usize = 16 * 1024;
/// Pending comments that nobody reviews are deleted after this many days (DynamoDB TTL).
pub const PENDING_TTL_DAYS: u64 = 30;

pub const MAX_NAME_CHARS: usize = 60;
pub const MAX_COMMENT_CHARS: usize = 2000;

pub struct App {
    pub store: Store,
    pub sns: aws_sdk_sns::Client,
    pub topic_arn: String,
    /// Signs moderation links and salts visitor hashes. Read once from SSM.
    pub secret: Vec<u8>,
}

fn env(name: &str) -> Result<String, Error> {
    std::env::var(name).map_err(|_| format!("missing environment variable {name}").into())
}

impl App {
    pub async fn from_env() -> Result<Self, Error> {
        let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        let secret = aws_sdk_ssm::Client::new(&config)
            .get_parameter()
            .name(env("SECRET_PARAM")?)
            .with_decryption(true)
            .send()
            .await?
            .parameter()
            .and_then(|p| p.value().map(|v| v.as_bytes().to_vec()))
            .ok_or("the secret parameter has no value")?;
        Ok(Self {
            store: Store::new(aws_sdk_dynamodb::Client::new(&config), env("TABLE_NAME")?),
            sns: aws_sdk_sns::Client::new(&config),
            topic_arn: env("TOPIC_ARN")?,
            secret,
        })
    }
}
