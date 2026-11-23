"""Step 4: checks every draft against lore's page format and prints what it
flags, by problem. Exits 1 when something is flagged."""

import re
import sys
from collections import Counter

from common import DRAFTS, LINK_LINE, MAX_EXAMPLES, MAX_URL, USAGE_LINE, file_name

KNOWN_SECTIONS = {"Usage", "Notes", "See also", "Categories", "Links"}
SEARCHES = re.compile(r"stackoverflow\.com/search|google\.[a-z.]+/search|geeksforgeeks\.org")  # lore adds these itself


def problems(path) -> list[str]:
    text = path.read_text(encoding="utf-8")
    lines = text.splitlines()
    found = []
    if re.search(r"[\x00-\x08\x0b-\x1f\x7f]", text):
        found.append("control character")
    if not lines or file_name(lines[0].removeprefix("# ").replace(" ", "-")) != path.name:
        found.append("first line is not `# name`")
    if len(lines) < 3 or not lines[2].strip() or lines[2].startswith("#"):
        found.append("no summary")
    section, examples = None, 0
    for line in lines[1:]:
        if line.startswith("## "):
            section = line[3:].strip()
            if section not in KNOWN_SECTIONS:
                found.append(f"unknown section {section}")
        elif section == "Usage" and line.strip():
            examples += 1
            if not USAGE_LINE.match(line):
                found.append("usage line format")
            elif "{{" in line:
                found.append("tldr placeholder left")
        elif section == "Links" and line.strip():
            link = LINK_LINE.match(line)
            if not link:
                found.append("link line format")
            elif len(link.group(2)) > MAX_URL:
                found.append("link too long")
            elif SEARCHES.search(link.group(2)):
                found.append("search link (lore adds those)")
    if examples > MAX_EXAMPLES:
        found.append("too many examples")
    return found


def main() -> None:
    pages = sorted(DRAFTS.glob("*.md"))
    flagged = {path.name: p for path in pages if (p := problems(path))}
    print(f"{len(pages)} pages, {len(flagged)} flagged")
    for problem, count in Counter(p for ps in flagged.values() for p in ps).most_common():
        print(f"  {count:>5}  {problem}")
    sys.exit(1 if flagged else 0)


if __name__ == "__main__":
    main()
