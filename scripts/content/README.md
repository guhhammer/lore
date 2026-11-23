# Generating content/

The scripts that make lore's pages, kept so the pages can be refreshed from
a newer tldr-pages, or a deleted page made again. Python 3, standard library
only. They draft in `.content-draft/` (gitignored, removed by `cli/clean`);
`merge.py`, `platforms.py` and `enrich.py` then write into `content/`.

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

| File | What it holds |
| --- | --- |
| `common.py` | paths, lore's page format (one writer), the tldr reader |
| `extra_pages.py` | pages written for lore directly |
| `categories.py` | category keywords, categories set by hand, the `danger` list |

- **Nothing is overwritten.** `merge.py` keeps every page already in
  `content/`, so edits made there survive; delete a page to have it made
  again. `enrich.py` rewrites only a page's `## See also` and
  `## Categories`, and never replaces a translation.
- **Python imports only from this folder**, never from the downloaded data.
- **Record the tldr commit** used in `content/README.md`, where tldr-pages is
  credited (CC BY 4.0). The pages in the repository were made from commit
  c90cc0b; the man-based ones also depend on the man pages of the machine
  they were made on.
