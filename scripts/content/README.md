# Generating content/

The scripts that made lore's command pages. You only need them to refresh
the pages from a newer tldr-pages, or to make a deleted page again; to fix
a page, just edit it in `content/`.

Python 3, standard library only. They draft in `.content-draft/`
(gitignored, removed by `cli/clean`); only `merge.py`, `platforms.py` and
`enrich.py` write into `content/`.

## Run them

From the repository root, in order:

```bash
mkdir -p .content-draft
git clone --depth 1 https://github.com/tldr-pages/tldr.git .content-draft/tldr-src
cargo run --release --example commands > .content-draft/commands.tsv

python3 scripts/content/tldr.py       # 1. tldr pages        -> .content-draft/pages/
python3 scripts/content/man.py        # 2. what tldr misses, from the man pages
python3 scripts/content/revise.py     # 3. add extra_pages.py, fill tldr's pointer pages
python3 scripts/content/validate.py   # 4. check the drafts' format
python3 scripts/content/merge.py      # 5. new drafts        -> content/
python3 scripts/content/platforms.py  # 6. Windows, BSD, SunOS, Android pages
python3 scripts/content/enrich.py     # 7. See also, categories, translations
cli/make                              #    the tests check every built-in page
```

## The weekly pages

`bundle.py` packs every tldr page (English and translated) in lore's
format into one zstd file, the one `lore --update-pages` fetches first.
The `content` workflow runs it each week; by hand, with the tldr checkout
above and the `zstd` program:

```bash
python3 scripts/content/bundle.py pages.zst
```

It writes only that file, never into `content/`.

## Good to know

- **Nothing is overwritten.** `merge.py` keeps every page already in
  `content/`, so edits made there survive; delete a page to have it made
  again. `enrich.py` changes only a page's `## See also` and
  `## Categories`, and never replaces a translation.
- **Record the tldr commit you used** in `content/README.md`, where
  tldr-pages is credited (CC BY 4.0). The current pages come from commit
  c90cc0b; the man-based ones also depend on the man pages of the machine
  they were made on.
- **The scripts import only from this folder**, never from the downloaded
  data.

| File | What it holds |
| --- | --- |
| `common.py` | Paths, lore's page format (one writer), the tldr reader. |
| `extra_pages.py` | Pages written for lore directly. |
| `categories.py` | Category keywords, categories set by hand, the `danger` list. |
| `bundle.py` | The weekly pages for `lore --update-pages` (see above). |
