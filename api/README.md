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

## Deploy

Everything runs in `us-east-2` with your AWS profile (not the GitHub Actions role, which cannot and should not edit
this stack). Needs the AWS CLI, Rust, and Python (for `cargo-lambda`, which bundles Zig to cross-compile for Lambda).

**One time**

```sh
# 1. The secret that signs the moderation links. The value is random and never printed or committed.
aws ssm put-parameter --region us-east-2 --name /cr0wg4n/comments/secret --type SecureString --value "$(openssl rand -hex 32)"

# 2. A private bucket where `package` uploads the compiled code (new buckets are private and encrypted).
aws s3 mb s3://<artifacts-bucket> --region us-east-2

# 3. cargo-lambda, in a local virtualenv (ignored by git).
python3 -m venv .venv && .venv/bin/pip install cargo-lambda
```

**Build, package and deploy** (from this folder; run the same commands again to update)

```sh
PATH="$PWD/.venv/bin:$PATH" cargo lambda build --release --x86-64 --output-format zip
aws cloudformation package --region us-east-2 --template-file template.yaml \
  --s3-bucket <artifacts-bucket> --output-template-file packaged.yaml
aws cloudformation deploy --region us-east-2 --template-file packaged.yaml --stack-name cr0wg4n-comments \
  --capabilities CAPABILITY_IAM --parameter-overrides OwnerEmail=<your-email>
```

**After the first deploy**

1. AWS e-mails `OwnerEmail` a *Subscription Confirmation*. Click **Confirm subscription** once (check spam). Until
   then no "comment pending" e-mails arrive.
2. Read the API address and put it in `blog/config/_default/params.toml` as `[comments] apiUrl`, then deploy the site:
   ```sh
   aws cloudformation describe-stacks --region us-east-2 --stack-name cr0wg4n-comments \
     --query 'Stacks[0].Outputs'
   ```
3. If the site's address changes, redeploy with `AllowedOrigin=https://<new-address>` (CORS allows one origin).

The Lambda reads `TABLE_NAME`, `SECRET_PARAM` and `TOPIC_ARN` from its environment; the stack sets them.

## Delete

```sh
# The stack: function, API, topic, role, logs. The comments table is KEPT on purpose (DeletionPolicy: Retain).
aws cloudformation delete-stack --region us-east-2 --stack-name cr0wg4n-comments
aws cloudformation wait stack-delete-complete --region us-east-2 --stack-name cr0wg4n-comments

# Optional, in any order. The first one deletes every stored comment and reaction for good.
aws dynamodb delete-table --region us-east-2 --table-name <TableName>            # name: see the stack's Outputs, or `aws dynamodb list-tables`
aws ssm delete-parameter --region us-east-2 --name /cr0wg4n/comments/secret
aws s3 rb s3://<artifacts-bucket> --force
```

Then remove `apiUrl` from `params.toml` (or leave it empty) and deploy the site, so the section stops rendering.

## License

[GNU GPL v3](../LICENSE), like the rest of the code in this repository.
