"""What the content scripts share: where things are, lore's page format, and
reading tldr-pages. Standard library only."""

from __future__ import annotations

import re
from dataclasses import dataclass, field
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
WORK = ROOT / ".content-draft"  # gitignored; removed by cli/clean
DRAFTS = WORK / "pages"
TLDR = WORK / "tldr-src"
COMMANDS = WORK / "commands.tsv"  # cargo run --example commands
CONTENT = ROOT / "content"

MAX_EXAMPLES = 8
MAX_URL = 76  # a longer URL is cut on narrow screens, and the terminal would open the cut one
TLDR_LINK = "https://tldr.inbrowser.app/pages/{platform}/{name}"

# tldr's English platforms, then the ones lore keeps for reference only
PLATFORMS = ["common", "linux", "osx"]
OTHER_SYSTEMS = {
    "windows": "Windows",
    "freebsd": "FreeBSD",
    "openbsd": "OpenBSD",
    "netbsd": "NetBSD",
    "sunos": "SunOS (Solaris)",
    "android": "Android",
}


# ── lore's page format (content/README.md) ──────────────────────────────────


@dataclass
class Page:
    title: str
    summary: str
    usage: list[tuple[str, str]] = field(default_factory=list)
    notes: list[str] = field(default_factory=list)
    links: list[tuple[str, str]] = field(default_factory=list)

    def render(self) -> str:
        lines = [f"# {self.title}", "", self.summary]

        def section(heading: str, body: list[str]) -> None:
            if body:
                lines.extend(["", f"## {heading}", *body])

        section("Usage", [f"- `{typed}` : {what}" for typed, what in self.usage[:MAX_EXAMPLES]])
        section("Notes", self.notes)
        section("Links", [f"- {label}: {url}" for label, url in self.links if fits(url)])
        return "\n".join(lines) + "\n"

    def save(self, folder: Path, name: str) -> None:
        folder.mkdir(parents=True, exist_ok=True)
        (folder / file_name(name)).write_text(self.render(), encoding="utf-8")


def read_page(path: Path) -> Page:
    """A page in lore's format; sections other than these four are dropped."""
    lines = path.read_text(encoding="utf-8").splitlines()
    title = lines[0].removeprefix("# ") if lines else path.stem
    summary, sections, current = [], {}, None
    for line in lines[1:]:
        if line.startswith("## "):
            current = sections.setdefault(line[3:].strip(), [])
        elif current is None:
            summary.append(line)
        elif line.strip():
            current.append(line)
    usage = [m.groups() for m in map(USAGE_LINE.match, sections.get("Usage", [])) if m]
    links = [m.groups() for m in map(LINK_LINE.match, sections.get("Links", [])) if m]
    return Page(title, " ".join(l.strip() for l in summary if l.strip()), usage,
                sections.get("Notes", []), links)


USAGE_LINE = re.compile(r"^- `([^`]+)` : (\S.*)$")
LINK_LINE = re.compile(r"^- ([^:\n]+): (https://\S+)$")


FORBIDDEN_IN_FILE_NAMES = '<>:"\\|?*'  # Windows refuses these; lore's page_key encodes them too


def file_name(name: str) -> str:
    """A page's file name, as lore's `page_key` looks for it: `:` → `%3A.md`."""
    return "".join(f"%{ord(c):02X}" if c in FORBIDDEN_IN_FILE_NAMES else c for c in name) + ".md"


def fits(url: str) -> bool:
    return url.startswith("https://") and len(url) <= MAX_URL


def sentence(text: str) -> str:
    """'list directory contents' → 'List directory contents.'"""
    text = text.strip().rstrip(".")
    return text[:1].upper() + text[1:] + "."


# ── tldr-pages ──────────────────────────────────────────────────────────────


@dataclass
class TldrPage:
    title: str
    summary: str
    examples: list[tuple[str, str]]
    more_info: str | None
    see_also: list[str]


def read_tldr(path: Path, english: bool = True) -> TldrPage:
    """One tldr page: `# title`, `> ` description lines, then `- what it
    does:` followed by `` `the command` ``. Among the description lines, a
    `<https://…>` is the "more information" link, and "See also: `x`" names
    related pages; other English lines, even with backticks, are summary.
    In a translation the labels are in its language, so any short
    "label: `x`" line counts as "see also"."""
    title, summary, examples, more_info, see_also = path.stem, [], [], None, []
    what = None
    for raw in path.read_text(encoding="utf-8").splitlines():
        line = "".join(c for c in raw if c.isprintable())
        if line.startswith("# "):
            title = line[2:].strip()
        elif line.startswith("> "):
            text = line[2:].strip()
            url = re.search(r"<(https://[^>]+)>", text)
            if english:
                # labelled lines only; any other line is summary, links and all
                link_line = text.startswith(("More information", "Homepage"))
                refers = text.startswith("See also")
            else:
                # "Mais informações: <https://…>": a label, then the link;
                # a link inside a sentence is part of the summary
                labelled = text.split("<https://")[0].rstrip().endswith((":", "："))
                link_line, refers = bool(url) and labelled, SEE_ALSO.match(text)
            if link_line:
                if url:
                    more_info = more_info or url.group(1).rstrip(".")
            elif refers:
                see_also += [ref for ref in re.findall(r"`([^`]+)`", text) if "{{" not in ref]
            else:
                summary.append(text)
        elif line.startswith("- "):
            what = describe(line[2:])
        elif what and len(line) > 2 and line[0] == line[-1] == "`":
            typed = fill_placeholders(line[1:-1]).strip()
            if "`" not in typed:
                examples.append((typed, what))
            what = None
    return TldrPage(title, " ".join(summary), examples, more_info, see_also)


SEE_ALSO = re.compile(r"^[^:：]{0,40}[:：]\s*`")  # "Ver também: `x`", in a translation


def fill_placeholders(text: str) -> str:
    """tldr's `{{path/to/file}}` → `path/to/file`; `{{[-a|--all]}}` → `-a`."""
    def plain(match: re.Match) -> str:
        inner = match.group(1)
        if inner.startswith("[") and inner.endswith("]") and "|" in inner:
            return inner[1:-1].split("|")[0]
        return inner
    return re.sub(r"\{\{(.*?)\}\}", plain, text)


def describe(text: str) -> str:
    """An example's description as lore writes it: tldr's `[c]reate` marks
    gone, no final colon or period, lower-case first letter, unless it starts
    an acronym or a lone letter (`URL`, `R` as in `chmod -R`)."""
    text = re.sub(r"\[([^\W\d_])\]", r"\1", text).strip().rstrip(":：.").strip()
    text = fill_placeholders(text)
    return text if text[:2].isupper() else text[:1].lower() + text[1:]


def tldr_files(language: str = "en", platforms=PLATFORMS):
    """(platform, path) of every tldr page in `language`, platform order."""
    folder = TLDR / ("pages" if language == "en" else f"pages.{language}")
    for platform in platforms:
        directory = folder / platform
        if directory.is_dir():
            for path in sorted(directory.glob("*.md")):
                yield platform, path


def tldr_links(page: TldrPage, platform: str, name: str) -> list[tuple[str, str]]:
    links = [("More information", page.more_info)] if page.more_info else []
    return links + [("tldr page", TLDR_LINK.format(platform=platform, name=name))]
