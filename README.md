# cr0wg4n.com

Personal site of Mauricio Matias C. (cr0wg4n), in English and Spanish. Built with [Hugo](https://gohugo.io) and the
[Blowfish](https://blowfish.page) theme, hosted on AWS (S3 + CloudFront).

```
blog/                the Hugo site (run every command from here)
  assets/css/        custom.css: all the site styling
  assets/img/        images used by pages
  config/_default/   hugo, params, languages (en/es), menus, deployment
  content/           pages: posts/, talks/, projects/, repos/ (each as .en.md and .es.md)
  layouts/           overrides of the theme (RSS, head extras, shortcodes)
  static/            files copied as-is (fonts, favicons, social card)
  themes/blowfish/   the theme, a git submodule
docs/                card-template.svg and make-card.py (article card generator)
.github/workflows/   deploy.yml, the automatic deploy
```

## Install

You need [Hugo](https://gohugo.io/installation/) **extended with deploy support** (0.168 or newer) and Git.

```sh
git clone --recurse-submodules git@github.com:cr0wg4n/cr0wg4n.github.io.git
cd cr0wg4n.github.io/blog
# forgot --recurse-submodules? the theme folder is empty: git submodule update --init --recursive
```

## Develop

```sh
hugo server -D        # http://localhost:1313 (English) and /es/ (Spanish); -D includes drafts
hugo --minify         # production build into blog/public/ (ignored by git)
```

To open it on your phone (same Wi-Fi): `hugo server -D --bind 0.0.0.0 --port 3000 --baseURL http://<LAN-IP>:3000`,
with that port open in your firewall.

## Add content

Each page exists in two languages with the same name: `slug.en.md` and `slug.es.md`.

- **Article, talk or project that lives elsewhere** (Medium, slides.com…): a short stub whose card links out.
  ```yaml
  ---
  title: "My experience at CLEI 2024"
  date: "2024-10-01"
  externalUrl: "https://cr0wg4n.medium.com/..."   # where the card links to
  featureimage: "img/posts/my-experience-at-clei-2024.jpg"
  tags: ["career"]
  summary: "Originally published on Medium."
  ---
  ```
  These stubs are `noindex`, and the RSS feed links to the original. New Medium posts appear in
  `https://medium.com/feed/@cr0wg4n`.
- **Post written for this site:** `hugo new content posts/my-post.en.md` (and `.es.md`) creates the skeleton from
  `archetypes/posts.md`: an answer-first `description`, then sections. It has no `externalUrl`, so it is indexed and
  gets `Article` structured data. Set `draft: false` to publish, and update `lastmod` when you change it (it becomes
  `dateModified`). To also publish it on Medium, wait until it is live here and use Medium's *Import a story* with
  the page URL: it sets the canonical link to this site, so search engines and AI answers credit `cr0wg4n.com`.
- **Projects** only know the year: `date: "2024-01-01"` plus `dateYearOnly: true`.
- **Images** go in `assets/img/<section>/`, referenced without a leading slash. Only add images you own or may use.
  For an article without a usable cover, make a title card (needs Pillow; the editable design is
  `docs/card-template.svg`):
  `python3 ../docs/make-card.py "My article" --source medium.com --out assets/img/posts/my-article.png`

Where to change other things: colors, fonts and effects in `assets/css/custom.css`; theme settings in
`config/_default/params.toml`; name, bio and links in `config/_default/languages.*.toml`; the link-preview image
(og:image) in `static/img/social-card.png`.

## Deploy

**Automatic:** every push to `main` that changes `blog/` runs `.github/workflows/deploy.yml`, which builds the site
and publishes it. It logs in to AWS with a temporary key (OIDC, role `github-actions-hugo-deploy`, only from this
repo's `main` branch), so no AWS keys are stored in GitHub. It can also be started from the Actions tab.

The role is defined in [`docs/aws-oidc-deploy.yml`](docs/aws-oidc-deploy.yml) (CloudFormation). To set it up on a
new account: `aws cloudformation deploy --region us-east-2 --template-file docs/aws-oidc-deploy.yml --stack-name
github-actions-deploy --capabilities CAPABILITY_NAMED_IAM`; to remove it, `aws cloudformation delete-stack`.
[`docs/aws-oidc-teardown.sh`](docs/aws-oidc-teardown.sh) removes a role created by hand instead.

**Manual** (settings in `blog/config/_default/deployment.toml`; needs the AWS CLI and a profile `cr0wg4n`):

```sh
cd blog
hugo --minify
AWS_PROFILE=cr0wg4n hugo deploy --dryRun --target production    # preview: what is uploaded and deleted
AWS_PROFILE=cr0wg4n hugo deploy --target production
```

It uploads only what changed, applies cache headers by file type and invalidates CloudFront. Hugo stops if a run
would delete more than 256 files. The bucket has versioning off, so deletions cannot be undone.

## License

Code: [GNU GPL v3](LICENSE). Original writing and images: [CC BY 4.0](CONTENT-LICENSE.md). Link-out entries keep the
terms of where they were published; Blowfish, Rubik and Font Awesome keep their own licenses.
