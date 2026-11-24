"""Step 7: completes every page in content/ and adds the translations.
- `## See also`: tldr's related commands.
- `## Categories`: set by hand, else by keywords (categories.py), plus
  `shell` for builtins and keywords, the system for other systems' pages,
  and `danger` first where it applies.
- content/i18n/<language>/: tldr's translations of the summary, the
  examples and the "more information" link; lore takes the rest of the
  page from the English one.
Rerunnable: it replaces the two sections it writes, and never overwrites a
translation."""

import re

from categories import DANGER, HAND, KEYWORDS
from common import (CONTENT, OTHER_SYSTEMS, PLATFORMS, ROOT, TLDR, Page, file_name,
                    read_tldr, tldr_files)

SYSTEM_TAGS = {"osx": "macos", "windows": "windows", "android": "android",
               "freebsd": "bsd", "openbsd": "bsd", "netbsd": "bsd", "sunos": "sunos"}
MAX_CATEGORIES = 3
MIN_SCORE = 3  # one mention in the summary, or three in the examples


def shell_words() -> set[str]:
    """The builtins and keywords lore knows, from its src/configs/."""
    config = "\n".join(p.read_text(encoding="utf-8") for p in sorted((ROOT / "src" / "configs").glob("*.rs")))
    blocks = re.findall(r"pub const \w+_(?:BUILTINS|KEYWORDS): &\[&str\] = &\[(.*?)\];", config, re.S)
    return {word for block in blocks for word in re.findall(r'"([^"]+)"', block)}


def by_keywords(text: str) -> list[str]:
    """The categories whose words a page uses most: three points each in the
    summary, one in the examples."""
    summary, _, usage = text.lower().partition("\n## usage")
    scores = {}
    for category, words in KEYWORDS.items():
        score = sum(3 * len(re.findall(rf"\b{re.escape(w)}\b", summary))
                    + len(re.findall(rf"\b{re.escape(w)}\b", usage)) for w in words)
        if score >= MIN_SCORE:
            scores[category] = score
    return sorted(scores, key=lambda c: (-scores[c], c))[:MAX_CATEGORIES]


def replace_sections(text: str, sections: dict[str, list[str]]) -> str:
    """`text` with these `##` sections removed and written again (when not
    empty) before `## Links`, or at the end."""
    kept, skipping = [], False
    for line in text.splitlines():
        if line.startswith("## "):
            skipping = line[3:].strip() in sections
        if not skipping:
            kept.append(line)
    text = "\n".join(kept).rstrip() + "\n"
    block = "\n\n".join(f"## {name}\n" + "\n".join(body) for name, body in sections.items() if body)
    if not block:
        return text
    head, links, tail = text.partition("\n## Links")
    return f"{head.rstrip()}\n\n{block}\n{links}{tail}" if links else f"{text.rstrip()}\n\n{block}\n"


def enrich_pages() -> None:
    see_also, system = {}, {}
    for platform, path in tldr_files(platforms=[*PLATFORMS, *OTHER_SYSTEMS]):
        system.setdefault(path.stem, platform)
        refs = [ref for ref in read_tldr(path).see_also if ref != path.stem]
        if refs:
            see_also.setdefault(path.stem, refs)

    shell = shell_words()
    for path in sorted(CONTENT.glob("*.md")):
        if path.name == "README.md":
            continue
        name = path.stem
        # the page without the sections written here, so its old categories
        # do not count as keywords for its new ones
        text = replace_sections(path.read_text(encoding="utf-8"), {"See also": [], "Categories": []})
        categories = [c.strip() for c in HAND[name].split(",")] if name in HAND else by_keywords(text)
        categories += ["shell"] * (name in shell) + [SYSTEM_TAGS.get(system.get(name), "")]
        if name in DANGER or name.startswith("mkfs."):
            categories.insert(0, "danger")
        categories = [c for c in dict.fromkeys(categories) if c]
        sections = {"See also": [f"- {ref}" for ref in see_also.get(name, [])],
                    "Categories": [", ".join(categories)] if categories else []}
        path.write_text(replace_sections(text, sections), encoding="utf-8")


def add_translations() -> int:
    made = 0
    for folder in sorted(TLDR.glob("pages.*")):
        language = folder.name.removeprefix("pages.")
        if language == "en":
            continue
        for platform, path in tldr_files(language, [*PLATFORMS, *OTHER_SYSTEMS]):
            english = CONTENT / file_name(path.stem)
            target = CONTENT / "i18n" / language / file_name(path.stem)
            if not english.exists() or target.exists():
                continue
            tldr = read_tldr(path, english=False)
            if tldr.summary:
                title = english.read_text(encoding="utf-8").splitlines()[0].removeprefix("# ")
                links = [("More information", tldr.more_info)] if tldr.more_info else []
                Page(title, tldr.summary, tldr.examples, links=links).save(target.parent, path.stem)
                made += 1
    return made


def main() -> None:
    enrich_pages()
    print(f"pages enriched, {add_translations()} new translations")


if __name__ == "__main__":
    main()
