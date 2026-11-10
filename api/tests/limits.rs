//! The tunable limits in src/config.rs must stay consistent with each other and with what real people do.
//! These are checked while compiling: changing a limit to a bad value stops the build of the tests.

use comments_api::config::*;

// 2000 characters of 4-byte emoji, a 60-character name, and generous room for the JSON around them.
const WORST_CASE_COMMENT_REQUEST: usize = MAX_COMMENT_CHARS * 4 + MAX_NAME_CHARS * 4 + 600;

const _: () = assert!(
    MAX_BODY_BYTES >= WORST_CASE_COMMENT_REQUEST,
    "a legitimate comment must fit in the request size limit"
);
const _: () = assert!(
    COMMENTS_PER_IP_PER_HOUR >= 3,
    "people behind one address must still be able to comment"
);
const _: () = assert!(
    COMMENTS_PER_HOUR_OVERALL > COMMENTS_PER_IP_PER_HOUR,
    "the overall cap must be above the per-address cap"
);
const _: () = assert!(
    REACTIONS_PER_IP_PER_HOUR >= 20,
    "reacting to a few pages is normal"
);
const _: () = assert!(
    MIN_FILL_TIME_MS >= 2_000 && MIN_FILL_TIME_MS <= 10_000,
    "the form cannot be sent faster than a person can type"
);
const _: () = assert!(
    NOTIFICATIONS_PER_HOUR < COMMENTS_PER_HOUR_OVERALL,
    "the inbox cap must be below the overall comment cap"
);
const _: () = assert!(PENDING_TTL_DAYS >= 7, "leave time to review");

#[test]
fn limits_are_consistent() {
    // The checks above run at compile time; this test only makes `cargo test` list them.
}
