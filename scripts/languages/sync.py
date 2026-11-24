"""Copies learnxinyminutes.com's pages into languages/, as they are, for
lore's languages topic.

The site is made from https://github.com/adambard/learnxinyminutes-docs
(CC BY-SA 3.0): one Markdown file per language, translations in folders
named by language (`de/`, `pt-br/`). Here they keep their file names; a
translation folder takes lore's language code (`pt-br` -> `pt_BR`), the one
lore reads from LANG, as content/i18n/ does.

languages/*.md and languages/i18n/ are replaced whole; README.md stays.
LICENSE.txt is copied with them. Standard library only. From the
repository root:

    git clone --depth 1 https://github.com/adambard/learnxinyminutes-docs.git .content-draft/lxy
    python3 scripts/languages/sync.py .content-draft/lxy
"""

from __future__ import annotations

import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "languages"
SKIPPED = {"README.md", "CONTRIBUTING.md"}


def lore_code(folder: str) -> str:
    """`pt-br` -> `pt_BR`, `de` -> `de`."""
    base, _, region = folder.partition("-")
    return f"{base}_{region.upper()}" if region else base


def main() -> None:
    if len(sys.argv) != 2:
        sys.exit(__doc__)
    source = Path(sys.argv[1])
    english = sorted(p for p in source.glob("*.md") if p.name not in SKIPPED)
    if not english:
        sys.exit(f"no pages in {source}")

    for old in OUT.glob("*.md"):
        if old.name != "README.md":
            old.unlink()
    shutil.rmtree(OUT / "i18n", ignore_errors=True)
    OUT.mkdir(exist_ok=True)

    for page in english:
        shutil.copyfile(page, OUT / page.name)
    translated = 0
    for folder in sorted(p for p in source.iterdir() if p.is_dir() and not p.name.startswith(".")):
        pages = sorted(folder.glob("*.md"))
        if not pages or folder.name in {"images", "lint"}:
            continue
        target = OUT / "i18n" / lore_code(folder.name)
        target.mkdir(parents=True, exist_ok=True)
        for page in pages:
            shutil.copyfile(page, target / page.name)
            translated += 1
    shutil.copyfile(source / "LICENSE.txt", OUT / "LICENSE.txt")

    commit = subprocess.run(["git", "-C", str(source), "rev-parse", "--short", "HEAD"],
                            capture_output=True, text=True).stdout.strip()
    print(f"{len(english)} pages and {translated} translations from commit {commit or '?'} -> {OUT}")


if __name__ == "__main__":
    main()
