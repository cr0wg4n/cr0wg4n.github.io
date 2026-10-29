# cr0wg4n.com

Personal site of Mauricio Matias C. (cr0wg4n), in English and Spanish. Built with [Hugo](https://gohugo.io) and the
[Blowfish](https://blowfish.page) theme, hosted on AWS (S3 + CloudFront).

- **Home:** profile and the latest posts.
- **Posts, Talks, Projects:** link-out entries that point to the original (Medium, slides.com, the project's own site).
- **Repos:** a selection of GitHub repositories.

```
.
├── blog/                      the Hugo site (run every command from here)
│   ├── assets/css/custom.css  all site styling (typography, components, glow, ripples)
│   ├── assets/img/            images used by pages (posts/, talks/, projects/, profile photo, logo)
│   ├── config/_default/       hugo, params, languages (en/es), menus, markup, deployment
│   ├── content/               pages: posts/, talks/, projects/, repos/ (each as .en.md and .es.md)
│   ├── layouts/               overrides of the theme (RSS, head extras, footer script, shortcodes)
│   ├── static/                files copied as-is (fonts, favicons, social card)
│   └── themes/blowfish/       the theme, a git submodule pinned to a release
├── docs/                      card-template.svg and make-card.py: the design behind the article cards
├── LICENSE                    GNU GPL v3 (the code)
└── CONTENT-LICENSE.md         CC BY 4.0 (the original writing and images)
```

## Requirements

| Tool | Why | Version |
|---|---|---|
| [Hugo](https://gohugo.io/installation/) **extended**, with deploy support | build, dev server, `hugo deploy` | 0.168 or newer (`hugo version` should say `extended` and `withdeploy`) |
| Git | the theme is a submodule | any recent |
| [AWS CLI v2](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) | deploying only | 2.x |
| Python 3 + Pillow | only for `docs/make-card.py` | `pip install pillow` |

No Node.js is needed: the theme ships its compiled CSS.

## Install

```sh
git clone --recurse-submodules git@github.com:cr0wg4n/cr0wg4n.github.io.git
cd cr0wg4n.github.io/blog
```

If you already cloned without `--recurse-submodules`, the theme folder is empty. Fix it with:

```sh
git submodule update --init --recursive
```

## Develop

From `blog/`:

```sh
hugo server -D          # http://localhost:1313, rebuilds and reloads on every change (-D includes drafts)
```

Open the English site at `/` and the Spanish one at `/es/`.

**See it on your phone** (same Wi-Fi). Use your computer's LAN address:

```sh
hugo server -D --bind 0.0.0.0 --port 3000 --baseURL http://<YOUR-LAN-IP>:3000
```

Then open `http://<YOUR-LAN-IP>:3000` on the phone. If it does not load: allow the port in your firewall
(for ufw: `sudo ufw allow from 192.168.1.0/24 to any port 3000 proto tcp`), and on Android give the browser the
**Nearby devices** permission.

Other useful commands:

```sh
hugo --minify                 # production build into blog/public/ (ignored by git)
hugo config                   # show the merged configuration
```

A warning about Blowfish and your Hugo version ("not compatible with this Hugo version") is harmless as long as the
site builds.

## Add content

Every page exists in two languages with the same file name: `slug.en.md` and `slug.es.md`.

### An article that lives elsewhere (Medium, dev-academy…)

`content/posts/<slug>.en.md` and `.es.md`:

```yaml
---
title: "My experience at CLEI 2024"
date: "2024-10-01"
externalUrl: "https://cr0wg4n.medium.com/..."      # where the card links to
featureimage: "img/posts/my-experience-at-clei-2024.jpg"
coverSource: "https://miro.medium.com/..."         # where the image came from (not displayed)
tags: ["career"]
summary: "Originally published on Medium."
---

Originally published on Medium: [My experience at CLEI 2024](https://cr0wg4n.medium.com/...)
```

Pages with `externalUrl` are *stubs*: the card links straight to the original, the stub page itself gets
`noindex`, and the RSS feed links to the original too. New Medium posts show up in
`https://medium.com/feed/@cr0wg4n`, which is handy to copy the title, date and cover from.

### A post written for this site

Same file layout, but **without `externalUrl`**: write the article in Markdown below the front matter. It is
indexed and carries the CC BY 4.0 license. `featureimage` is optional.

### A talk or a project

Same idea in `content/talks/` and `content/projects/`: `externalUrl` to the slides or the project's site,
`featureimage: "img/talks/<slug>.png"` (or `img/projects/…`), `tags`, a short `summary`. Projects only know the
year, so they use `date: "2024-01-01"` plus `dateYearOnly: true` to show just "2024".

### Images and cards

Put images in `assets/img/<section>/` and reference them without a leading slash (`img/posts/foo.png`). Hugo
resizes them. **Only add images you own or have permission to use** (see `CONTENT-LICENSE.md`). For an article
without a usable cover, generate a card from its title:

```sh
python3 ../docs/make-card.py "My new article" --source medium.com --out assets/img/posts/my-new-article.png
```

Run it from `blog/`. The first run downloads the Rubik font into `docs/.cache/` (ignored by git). The same design is
available as an editable SVG in `docs/card-template.svg`. Cards are 1200×600 (the 2:1 ratio of the thumbnails). Do not put emoji
in the title: the font has none.

## Look and feel

- **Styling** lives in `assets/css/custom.css`, split into numbered sections. The ripples and the top glow are
  controlled by variables near the top of their sections (`--ripple-stroke`, `--ripple-alpha`, `--glow-alpha`…).
- **Theme settings** (color scheme `fire`, dark mode by default, card thumbnail ratio) are in
  `config/_default/params.toml`. Profile name, headline, bio and links are in `config/_default/languages.*.toml`;
  the home text is `content/_index.*.md`.
- **Fonts and icons** are self-hosted: Rubik in `static/fonts/` (SIL OFL, licence included), favicons in `static/`.
- **Social preview** image: `static/img/social-card.png` (1200×630). Replace it when your photo or title changes.
- **Updating the theme:** `cd themes/blowfish && git fetch --tags && git checkout <tag>`, then commit the new
  submodule pointer from the repo root. Check the site afterwards: some overrides in `layouts/` copy theme files.

## Deploy (AWS S3 + CloudFront)

The site is served by CloudFront (distribution `E2VL3GBGM7ES3F`) from the S3 bucket `cr0wg4n-website`
(`us-east-2`, static website hosting). The settings are in `blog/config/_default/deployment.toml`.

### One-time setup

1. **Credentials.** Create an IAM user with an access key that can `s3:ListBucket`, `s3:PutObject`,
   `s3:DeleteObject` on the bucket and `cloudfront:CreateInvalidation` on the distribution. Then:
   ```sh
   aws configure --profile cr0wg4n        # access key, secret, region us-east-2, output json
   aws sts get-caller-identity --profile cr0wg4n
   ```
2. **404 page.** Make the bucket serve `404.html` for unknown URLs:
   ```sh
   aws s3api put-bucket-website --bucket cr0wg4n-website --profile cr0wg4n \
     --website-configuration '{"IndexDocument":{"Suffix":"index.html"},"ErrorDocument":{"Key":"404.html"}}'
   ```

### Publish

```sh
cd blog
hugo --minify
AWS_PROFILE=cr0wg4n hugo deploy --dryRun --target production     # preview: what is uploaded and deleted
AWS_PROFILE=cr0wg4n hugo deploy --target production              # for real
```

`hugo deploy` uploads only what changed (it compares checksums), deletes what you removed, applies cache headers by
file type, and invalidates CloudFront at the end. As a safety brake, Hugo refuses to delete more than 256 files in one
run (`--maxDeletes`); a normal deploy removes only a handful, so if it stops there, check what changed before
overriding it with `--maxDeletes -1`.

Cache rules (`deployment.toml`): fingerprinted CSS/JS and resized images for a year; fonts 30 days; HTML, feeds and
JSON are always revalidated; everything else one hour. The bucket has **versioning off**, so a deletion cannot be
undone: back up first if in doubt (`aws s3 sync s3://cr0wg4n-website ./backup --profile cr0wg4n`).

### Automatic deploy (GitHub Actions)

Every push to `main` that touches `blog/` runs `.github/workflows/deploy.yml`: it installs Hugo (checking the
download's checksum), builds the site and runs `hugo deploy`, the same commands as above. You can also start it
by hand from the **Actions** tab (*Run workflow*).

AWS access uses OIDC, so **no AWS keys are stored in GitHub**: the job asks AWS for a temporary key through the
role `github-actions-hugo-deploy`. Its trust policy only accepts runs from the `main` branch of this repository,
and its permissions cover only the bucket and the distribution. One-time AWS setup (it must exist before the
first run, or the AWS step fails and nothing is deployed):

1. An IAM identity provider for `token.actions.githubusercontent.com` (audience `sts.amazonaws.com`).
2. The role `github-actions-hugo-deploy`, trusting `repo:cr0wg4n/cr0wg4n.github.io:ref:refs/heads/main`.
3. An inline policy on the role: `s3:ListBucket`, `s3:GetObject`, `s3:PutObject`, `s3:DeleteObject` on the bucket
   and `cloudfront:CreateInvalidation` on the distribution.

The third-party actions in the workflow are pinned to full commit SHAs. To update them, pick the new tag's commit
SHA on purpose and change it in the file.

## SEO and feeds

- Every page has a canonical URL, `en`/`es`/`x-default` hreflang links, Open Graph and Twitter tags (default image:
  the social card) and a `rel="license"` link. The home page adds `Person` structured data.
- RSS: `/index.xml` (posts only), plus one feed per section and language. Items link to the original article.
  The template is `layouts/_default/rss.xml`.
- The sitemap lists only pages with original content; link-out stubs are `noindex`.

## Troubleshooting

| Symptom | Fix |
|---|---|
| Empty page, theme missing | `git submodule update --init --recursive` |
| `address already in use` | change `--port`, or stop whatever holds it (`ss -ltnp \| grep :3000`) |
| Phone cannot open the LAN address | firewall rule for the port; Nearby devices permission (Android); same Wi-Fi |
| `hugo deploy` says no credentials | `aws configure --profile cr0wg4n`, and run it with `AWS_PROFILE=cr0wg4n` |
| Old page after a deploy | CloudFront cache: the deploy invalidates it, wait a minute and hard-refresh |

## License

- **Code** (templates, CSS, scripts): [GNU GPL v3](LICENSE).
- **Original writing and images**: [CC BY 4.0](CONTENT-LICENSE.md). Credit "Mauricio Matias C. (cr0wg4n), cr0wg4n.com".
- Link-out entries keep the terms of where they were published. Third-party components (Blowfish, Rubik,
  Font Awesome) keep their own licenses.
