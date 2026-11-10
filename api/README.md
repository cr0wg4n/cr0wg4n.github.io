# comments-api

Reactions and comments for [cr0wg4n.com](https://cr0wg4n.com): a Rust AWS Lambda behind an HTTP API, with DynamoDB
for storage and SNS to e-mail the owner. Pay-per-use. The site side lives in `../blog/layouts/partials/comments.html`
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
                                                   --> SNS        "comment pending" e-mail (topic with an e-mail subscription)
                                                   --> SSM        signing secret (SecureString)
```

- A new comment is `pending` and invisible. The e-mail has *Approve* and *Delete* links signed with HMAC-SHA256.
  They open a page with a button: mail scanners that open links therefore cannot change anything.
- Spam and flooding, in layers (all limits are constants in `src/config.rs`):
  1. API Gateway throttling per route (comments: 2 requests/s, bursts of 5): excess gets a 429 and never runs the Lambda.
  2. Per address: 5 comments and 60 reactions per hour. People behind one address still fit.
  3. Overall: 120 comments per hour, and at most 20 "pending" e-mails per hour. Comments over the e-mail cap are
     still stored, and one summary e-mail says so; the inbox cannot be flooded.
  4. A honeypot field and a minimum fill time (3 s) for simple bots, answered like a real submission.
  5. Requests over 16 KB are rejected; text is limited to 60 and 2000 characters; pages must be `/posts/<slug>/`.
  6. Nothing is public until approved, and unreviewed pending comments are deleted after 30 days.
- Privacy: no raw IP address is stored. Addresses and browsers are salted one-way hashes that change every day, kept
  in DynamoDB items that expire through TTL (hours to two days).
- Table layout (one table, keys `PK`/`SK`, TTL attribute `ttl`) is documented at the top of `src/store.rs`.

## Layout

```
src/
  main.rs            entry point: builds the shared state and starts the Lambda
  lib.rs             module list
  config.rs          environment, shared state, and every limit
  http.rs            read requests (IP, host, body), build JSON and HTML responses
  routes/            one file per endpoint: comments.rs, reactions.rs, moderation.rs (mod.rs dispatches)
  notify.rs          the "comment pending" e-mail through SNS, with the hourly cap
  security/          signing.rs (HMAC, hashes), validation.rs (paths, text), escape.rs (HTML)
  store/             DynamoDB: comments.rs, reactions.rs, limits.rs (rate-limit counters); layout in mod.rs
tests/               one file per area: signing, validation, escape, limits
dev/                 mock-server.py: a fake API to try the site locally
template.yaml        CloudFormation stack: table, topic, function, role, HTTP API, routes, throttling
```

## Try the site locally

The comment section only renders when `[comments] apiUrl` is set, so point it at the fake API in `dev/`:

```sh
python3 api/dev/mock-server.py                                              # terminal 1
cd blog && HUGO_PARAMS_COMMENTS_APIURL=http://127.0.0.1:8787 hugo server -D  # terminal 2
```

Open `http://localhost:1313/posts/masters-of-sampling-miloj-daft-punk/` and scroll to the end. To try the real stack
instead, deploy it with `AllowedOrigin=http://localhost:1313` (CORS only allows one origin) and use its `ApiUrl`.

## Build and deploy

One-time setup (signing secret, artifacts bucket; the e-mail subscription is confirmed after the first deploy) is in the [root README](../README.md), section
*Reactions and comments*. Then, from this folder:

```sh
pip install cargo-lambda      # once; bundles Zig, which cross-compiles for Lambda
cargo lambda build --release --x86-64 --output-format zip
aws cloudformation package --region us-east-2 --template-file template.yaml \
  --s3-bucket <your-artifacts-bucket> --output-template-file packaged.yaml
aws cloudformation deploy --region us-east-2 --template-file packaged.yaml --stack-name cr0wg4n-comments \
  --capabilities CAPABILITY_IAM --parameter-overrides OwnerEmail=<you>
```

The Lambda reads `TABLE_NAME`, `SECRET_PARAM` and `TOPIC_ARN` from its environment; the stack sets them.
The table is kept if the stack is deleted, because it holds the comments.

## License

[GNU GPL v3](../LICENSE), like the rest of the code in this repository.
