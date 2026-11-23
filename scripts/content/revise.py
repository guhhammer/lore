"""Step 3: revises the drafts. Adds the pages of extra_pages.py (keeping a
draft's links), and fills tldr's pointer pages, whose only example is
"see `tldr other`", with the examples of the pages they point to."""

from common import DRAFTS, MAX_EXAMPLES, Page, read_page
from extra_pages import PAGES


def is_pointer(page: Page) -> bool:
    return bool(page.usage) and all(typed.startswith("tldr ") for typed, _ in page.usage)


def main() -> None:
    for name, (summary, usage, notes) in PAGES.items():
        path = DRAFTS / f"{name}.md"
        links = read_page(path).links if path.exists() else []
        Page(name, summary, usage, [notes] if notes else [], links).save(DRAFTS, name)

    filled = 0
    for path in sorted(DRAFTS.glob("*.md")):
        page = read_page(path)
        if not is_pointer(page):
            continue
        targets = [typed.split()[-1] for typed, _ in page.usage]
        per_target = MAX_EXAMPLES if len(targets) == 1 else 2
        borrowed = []
        for target in targets:
            target_path = DRAFTS / f"{target}.md"
            if target != path.stem and target_path.exists():
                target_page = read_page(target_path)
                if not is_pointer(target_page):
                    borrowed += target_page.usage[:per_target]
        if borrowed:
            page.usage = borrowed
            page.save(DRAFTS, path.stem)
            filled += 1
    print(f"{len(PAGES)} pages written for lore, {filled} pointer pages filled")


if __name__ == "__main__":
    main()
