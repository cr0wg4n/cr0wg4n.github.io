# comments-api

Reactions and comments for [cr0wg4n.com](https://cr0wg4n.com): a Rust AWS Lambda behind an HTTP API, with DynamoDB
for storage and SES to e-mail the owner. Pay-per-use. The site side lives in `../blog/layouts/partials/comments.html`
and `../blog/assets/js/comments.js`.

## Routes

| Route | What it does |
|---|---|
| `GET /comments?path=/posts/x/` | Approved comments of a page, oldest first |
| `POST /comments` | `{path, name, body, website, ts}`: stores a *pending* comment and e-mails the owner. Always answers `202` |
| `GET /reactions?path=...` | Counters of the four reactions: `like`, `love`, `idea`, `party` |
| `POST /reactions` | `{path, kind}`: one reaction per visitor, kind and page per day |
| `GET /moderate?action=&p=&k=&sig=` | Confirmation page opened from the e-mail. Changes nothing |
| `POST /moderate` | Approves or deletes, only with a valid signature |

`path` must be a page under `/posts/` or `/es/posts/`. CORS is configured in API Gateway (`template.yaml`), only for
the site's origin.

## How it works

```
browser --> HTTP API (throttled) --> Lambda (Rust) --> DynamoDB   comments, counters, rate limits
                                                   --> SES        "comment pending" e-mail
                                                   --> SSM        signing secret (SecureString)
```

- A new comment is `pending` and invisible. The e-mail has *Approve* and *Delete* links signed with HMAC-SHA256.
  They open a page with a button: mail scanners that open links therefore cannot change anything.
- Spam: a honeypot field, a minimum fill time (3 s), 3 comments and 30 reactions per visitor per hour, and the
  API-wide throttle (10 requests/s, bursts of 20).
- Privacy: no raw IP address is stored. A visitor is a salted one-way hash that changes every day, kept in
  DynamoDB items that expire through TTL (hours to two days).
- Table layout (one table, keys `PK`/`SK`, TTL attribute `ttl`) is documented at the top of `src/store.rs`.

## Layout

```
src/lib.rs        exposes the modules below
src/security.rs   signing, hashing, path and text validation, HTML escaping (pure, no AWS)
src/store.rs      all DynamoDB access
src/main.rs       Lambda entry point: routing, handlers, moderation e-mail and pages
dev/              mock-server.py: a fake API to try the site locally
tests/            integration tests, one file per module (tests/security.rs)
template.yaml     CloudFormation stack: table, function, role, HTTP API, routes, throttling
```

## Develop

```sh
cargo test            # unit-level tests in tests/
cargo clippy --all-targets
```

The tests cover the pure helpers. The handlers and the DynamoDB code need real AWS services, so they are checked
after a deploy (try the routes with `curl` against the stack's `ApiUrl`).

## Try the site locally

The comment section only renders when `[comments] apiUrl` is set, so point it at the fake API in `dev/`:

```sh
python3 api/dev/mock-server.py                                              # terminal 1
cd blog && HUGO_PARAMS_COMMENTS_APIURL=http://127.0.0.1:8787 hugo server -D  # terminal 2
```

Open `http://localhost:1313/posts/masters-of-sampling-miloj-daft-punk/` and scroll to the end. To try the real stack
instead, deploy it with `AllowedOrigin=http://localhost:1313` (CORS only allows one origin) and use its `ApiUrl`.

## Build and deploy

One-time setup (SES identity, signing secret, artifacts bucket) is in the [root README](../README.md), section
*Reactions and comments*. Then, from this folder:

```sh
pip install cargo-lambda      # once; bundles Zig, which cross-compiles for Lambda
cargo lambda build --release --x86-64 --output-format zip
aws cloudformation package --region us-east-2 --template-file template.yaml \
  --s3-bucket <your-artifacts-bucket> --output-template-file packaged.yaml
aws cloudformation deploy --region us-east-2 --template-file packaged.yaml --stack-name cr0wg4n-comments \
  --capabilities CAPABILITY_IAM --parameter-overrides OwnerEmail=<you> FromEmail=<you>
```

The Lambda reads `TABLE_NAME`, `SECRET_PARAM`, `OWNER_EMAIL` and `FROM_EMAIL` from its environment; the stack sets them.
The table is kept if the stack is deleted, because it holds the comments.

## License

[GNU GPL v3](../LICENSE), like the rest of the code in this repository.
