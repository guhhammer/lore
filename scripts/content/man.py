"""Step 2: drafts for the commands lore lists (commands.tsv) that tldr does
not cover, from their local man pages: the summary from `whatis`, the forms
from SYNOPSIS, then options whose description is short, and a documentation
link when the page gives one (not a bug tracker or mailing list). Commands
with no man page are listed in .content-draft/no_page.txt."""

import os
import re
import subprocess

from common import COMMANDS, DRAFTS, WORK, Page, fits, sentence

SECTIONS = "1,8,6,5,7"  # user commands first, then admin, games, files, misc
GENERAL_FORM = "the general form, as the manual gives it"
OTHER_FORM = "another form of the same command"
MAX_FORMS = 5
MAX_FORM_LENGTH = 110
MIN_EXAMPLES = 3  # options are added until there are this many lines
MAX_OPTION_TEXT = 70
NOT_DOCUMENTATION = re.compile(r"issue|bug|mailman|mailing|lists\.|report")


def run(*args: str, stdin: str | None = None) -> str:
    env = {**os.environ, "MANWIDTH": "160", "LC_ALL": "C.UTF-8"}
    try:
        return subprocess.run(args, input=stdin, capture_output=True, text=True, timeout=20, env=env).stdout
    except (OSError, subprocess.TimeoutExpired):
        return ""


def summary(name: str) -> str | None:
    """`whatis`'s description, preferring the entry for this exact name."""
    entries = [re.match(r"^(\S+) \((\w+)\)\s+- (.*)$", line) for line in run("whatis", "-s", SECTIONS, name).splitlines()]
    entries = [m for m in entries if m]
    exact = [m for m in entries if m.group(1) == name]
    best = (exact or entries or [None])[0]
    return best.group(3).strip() if best else None


def forms(name: str, manual: str) -> list[str]:
    """The command lines under SYNOPSIS, one per form, continuation lines joined."""
    synopsis, inside = [], False
    for line in manual.splitlines():
        if line.startswith("SYNOPSIS"):
            inside = True
        elif inside and line[:1].strip():
            break  # the next heading
        elif inside and line.strip():
            synopsis.append(line.strip())
    found = []
    for line in synopsis:
        if line.startswith(name) or not found:
            found.append(line)
        else:
            found[-1] += " " + line
    found = [re.sub(r"\s+", " ", f) for f in found]
    return [f for f in found if len(f) <= MAX_FORM_LENGTH and "`" not in f and f.isascii()][:MAX_FORMS]


def options(name: str, manual: str) -> list[tuple[str, str]]:
    """(`name -x`, what -x does) for each option under OPTIONS whose
    description fits on one short line."""
    found, flag, inside = [], None, False
    for line in manual.splitlines():
        if line.startswith("OPTIONS"):
            inside = True
            continue
        if inside and line[:1].strip():
            break  # the next heading
        option = re.match(r"^\s{3,8}(-[\w-]+),?(\s{2,}(\S.*))?$", line)
        if inside and option:
            flag = option.group(1)
            if option.group(3):  # the description on the same line
                found.append((flag, option.group(3)))
                flag = None
        elif inside and flag and line.strip():
            found.append((flag, line.strip()))
            flag = None
    short = []
    for flag, text in found:
        text = text.split(". ")[0].rstrip(".")
        if len(text) <= MAX_OPTION_TEXT:
            short.append((f"{name} {flag}", text[:1].lower() + text[1:]))
    return short


def main() -> None:
    listed = [line.rstrip("\n").split("\t") for line in COMMANDS.read_text().splitlines()]
    drafted = {path.stem for path in DRAFTS.glob("*.md")}
    missing, made = [], 0
    for name, kind in listed:
        if kind != "Program" or name in drafted:
            continue
        description = summary(name)
        if not description:
            missing.append(name)
            continue
        manual = run("col", "-bx", stdin=run("man", "-S", SECTIONS, name))
        usage = [(form, GENERAL_FORM if i == 0 else OTHER_FORM) for i, form in enumerate(forms(name, manual))]
        usage += options(name, manual)[: max(0, MIN_EXAMPLES - len(usage))]
        urls = (url.rstrip(".,;") for url in re.findall(r"https://[^\s<>)\]\"]+", manual))
        links = [("Documentation", url) for url in urls if fits(url) and not NOT_DOCUMENTATION.search(url)][:1]
        Page(name, sentence(description), usage, links=links).save(DRAFTS, name)
        made += 1
    (WORK / "no_page.txt").write_text("\n".join(missing) + "\n")
    print(f"{made} pages from man pages, {len(missing)} commands with none")


if __name__ == "__main__":
    main()
