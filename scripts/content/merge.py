"""Step 5: copies the drafts into content/. A page already in content/ is
kept, so edits made there survive: delete a page first to regenerate it."""

import shutil

from common import CONTENT, DRAFTS, file_name


def main() -> None:
    existing = {path.name for path in CONTENT.glob("*.md")}
    new = [path for path in sorted(DRAFTS.glob("*.md")) if file_name(path.stem) not in existing]
    for path in new:
        shutil.copy(path, CONTENT / file_name(path.stem))
    print(f"{len(new)} new pages in content/, {len(existing)} kept")


if __name__ == "__main__":
    main()
