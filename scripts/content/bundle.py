"""The weekly pages `lore --update-pages` fetches first: every tldr page,
English and translated, in lore's format, packed into one zstd file of
records (a 0x1E character, the path, a newline, the page), the format
src/info/updates.rs reads. lore keeps only what it lacks.

Needs the tldr checkout (see README.md) and the `zstd` program. From the
repository root:

    python3 scripts/content/bundle.py OUT.zst
"""

from __future__ import annotations

import subprocess
import sys
import tempfile
from pathlib import Path

from common import PLATFORMS, TLDR, Page, file_name, read_tldr, tldr_links

RECORD_MARK = "\x1e"  # src/configs/: RECORD_MARK


def pages(language: str):
    """(name, page text) for one language, the first platform winning."""
    folder = TLDR / ("pages" if language == "en" else f"pages.{language}")
    seen = set()
    for platform in PLATFORMS:
        for path in sorted((folder / platform).glob("*.md")):
            name = path.stem
            if name in seen:
                continue
            tldr = read_tldr(path, english=language == "en")
            if not tldr.summary:
                continue
            title = tldr.title if tldr.title.replace(" ", "-") == name else name
            seen.add(name)
            yield name, Page(title, tldr.summary, tldr.examples,
                             links=tldr_links(tldr, platform, name)).render()


def main() -> None:
    if len(sys.argv) != 2:
        sys.exit(__doc__)
    out = Path(sys.argv[1])
    translated = {p.name.removeprefix("pages.") for p in TLDR.glob("pages.*")} - {"en"}
    languages = ["en", *sorted(translated)]
    count = 0
    with tempfile.NamedTemporaryFile("w", encoding="utf-8", suffix=".txt", delete=False) as raw:
        for language in languages:
            prefix = "" if language == "en" else f"i18n/{language}/"
            for name, page in pages(language):
                raw.write(f"{RECORD_MARK}{prefix}{file_name(name)}\n{page}")
                count += 1
    subprocess.run(["zstd", "-19", "-q", "-f", raw.name, "-o", str(out)], check=True)
    Path(raw.name).unlink()
    print(f"{count} pages in {len(languages)} languages -> {out}")


if __name__ == "__main__":
    main()
