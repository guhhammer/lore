"""Step 1: tldr-pages (English: common, linux, osx) → drafts in
.content-draft/pages/. The first platform to have a name wins."""

from common import DRAFTS, Page, read_tldr, tldr_files, tldr_links


def main() -> None:
    written = set()
    for platform, path in tldr_files():
        name = path.stem
        if name in written:
            continue
        tldr = read_tldr(path)
        if not tldr.summary:
            continue
        # a subcommand keeps tldr's title: `git commit` in git-commit.md
        title = tldr.title if tldr.title.replace(" ", "-") == name else name
        Page(title, tldr.summary, tldr.examples, links=tldr_links(tldr, platform, name)).save(
            DRAFTS, name
        )
        written.add(name)
    print(f"{len(written)} pages from tldr")


if __name__ == "__main__":
    main()
