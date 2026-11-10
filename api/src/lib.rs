//! Comments and reactions API for cr0wg4n.com. The Lambda entry point is in main.rs; the tests are in tests/.
//!
//!   config    environment, shared state and every tunable limit
//!   http      small helpers to read requests and build responses
//!   routes    one handler per endpoint
//!   notify    the "comment pending" e-mail (SNS)
//!   security  signing, hashing, validation and escaping (pure, no AWS)
//!   store     all DynamoDB access

pub mod config;
pub mod http;
pub mod notify;
pub mod routes;
pub mod security;
pub mod store;
