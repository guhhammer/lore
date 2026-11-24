"""Add a Wikipedia link to every English page whose command or program has an
article on English Wikipedia.

Looks each name up with the MediaWiki API (50 titles per request), accepts an
article only when its intro describes software, and appends
`- Wikipedia: URL` to the page's `## Links` section (creating it if missing).
A subcommand page (`git commit`) gets its parent program's article. Answers are
cached in `.content-draft/wikipedia-cache.json`; the matches are listed in
`.content-draft/wikipedia-review.tsv` for a human to check. Running it again
changes nothing.

    python3 scripts/content/wikipedia.py      # from the repository root
"""

from __future__ import annotations

import json
import re
import sys
import time
import urllib.error
import urllib.parse
import urllib.request

from common import CONTENT, MAX_URL, WORK

API = "https://en.wikipedia.org/w/api.php"
USER_AGENT = "lore-content-script/1.0"
CACHE = WORK / "wikipedia-cache.json"
REVIEW = WORK / "wikipedia-review.tsv"
BATCH = 50
PAUSE = 0.5
TRIES = 5
SUFFIXES = [" (Unix)", " (command)", " (software)", " (programming language)", ""]
SOFTWARE_WORDS = [
    "command", "utility", "program", "software", "unix", "linux", "shell", "tool",
    "library", "compiler", "interpreter", "daemon", "protocol", "package manager",
    "terminal", "operating system", "editor", "framework", "file format",
    "programming language", "emulator", "server", "client",
]
URL_BASE = "https://en.wikipedia.org/wiki/"


def load_cache() -> dict:
    if CACHE.exists():
        return json.loads(CACHE.read_text(encoding="utf-8"))
    return {}


def fetch(params: dict) -> dict:
    url = API + "?" + urllib.parse.urlencode(params)
    for attempt in range(TRIES):
        time.sleep(PAUSE)
        try:
            request = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
            with urllib.request.urlopen(request, timeout=60) as response:
                return json.load(response)
        except urllib.error.HTTPError as err:
            print(f"HTTP {err.code}, waiting", file=sys.stderr, flush=True)
            if err.code != 429 and err.code < 500:
                break  # a request Wikipedia refuses will be refused again
        except (urllib.error.URLError, TimeoutError) as err:
            print(f"{err}, waiting", file=sys.stderr, flush=True)
        time.sleep(5 * (attempt + 1))
    raise RuntimeError("Wikipedia did not answer: " + url[:100])


def lookup(titles: list[str], cache: dict) -> None:
    """Ask for `titles` (not already cached); cache[title] = article or None."""
    todo = sorted({t for t in titles if t not in cache})
    for start in range(0, len(todo), BATCH):
        batch = todo[start:start + BATCH]
        params = {
            "action": "query", "format": "json", "formatversion": "2", "redirects": "1",
            "prop": "extracts|pageprops", "exintro": "1", "explaintext": "1",
            "exsentences": "2", "exlimit": "max", "ppprop": "disambiguation",
            "titles": "|".join(batch),
        }
        pages, normalized, redirects = {}, {}, {}
        while True:
            try:
                data = fetch(params)
            except RuntimeError:
                if len(batch) == 1:  # one title nobody can answer: no article
                    break
                for half in (batch[:len(batch) // 2], batch[len(batch) // 2:]):
                    lookup(half, cache)
                batch = []
                break
            query = data.get("query", {})
            for item in query.get("normalized", []):
                normalized[item["from"]] = item["to"]
            for item in query.get("redirects", []):
                redirects[item["from"]] = item["to"]
            for page in query.get("pages", []):
                old = pages.get(page["title"], {})
                # a continued answer fills in the fields the first one lacked
                pages[page["title"]] = {**old, **{k: v for k, v in page.items() if v is not None}}
            if "continue" not in data:
                break
            params = {**params, **data["continue"]}
        for title in batch:
            final = normalized.get(title, title)
            for _ in range(5):  # a redirect may chain
                if final not in redirects:
                    break
                final = redirects[final]
            page = pages.get(final)
            if not page or page.get("missing") or "disambiguation" in page.get("pageprops", {}):
                cache[title] = None
            else:
                cache[title] = {"title": page["title"], "extract": page.get("extract", "") or ""}
        CACHE.parent.mkdir(parents=True, exist_ok=True)
        CACHE.write_text(json.dumps(cache, ensure_ascii=False, indent=0), encoding="utf-8")
        print(f"{start + len(batch)}/{len(todo)}", file=sys.stderr, flush=True)


def accepted(article: dict, name: str, bare: bool) -> bool:
    text = article["extract"].lower()
    if not any(word in text for word in SOFTWARE_WORDS):
        return False
    if bare:
        return name.lower() in text or article["title"].lower() == name.lower()
    return True


def find(name: str, cache: dict):
    """(candidate, article) for the first accepted candidate, else None."""
    for suffix in SUFFIXES:
        candidate = name + suffix
        article = cache.get(candidate)
        if article and accepted(article, name, suffix == ""):
            return candidate, article
    return None


def url_of(title: str) -> str:
    return URL_BASE + urllib.parse.quote(title.replace(" ", "_"), safe="()_,'-.:")


def add_link(text: str, url: str) -> str:
    line = f"- Wikipedia: {url}"
    lines = text.split("\n")
    for i, current in enumerate(lines):
        if current.strip() == "## Links":
            end = i + 1
            while end < len(lines) and not lines[end].startswith("## "):
                end += 1
            last = end
            while last > i + 1 and not lines[last - 1].strip():
                last -= 1
            lines.insert(last, line)
            return "\n".join(lines)
    return text.rstrip("\n") + "\n\n## Links\n" + line + "\n"


def main() -> int:
    pages = {}
    for path in sorted(CONTENT.glob("*.md")):
        if path.name == "README.md":
            continue
        text = path.read_text(encoding="utf-8")
        first = text.split("\n", 1)[0]
        title = first[2:].strip() if first.startswith("# ") else path.stem
        pages[path] = (text, title.split()[0] if title.split() else path.stem)

    # characters a Wikipedia title cannot hold (`|` would also split a batch)
    pages = {p: (t, n) for p, (t, n) in pages.items() if not set(n) & set("#<>[]|{}_")}
    cache = load_cache()
    wanted = {name + s for _, name in pages.values() for s in SUFFIXES}
    print(f"asking Wikipedia about {len(wanted - set(cache))} titles", file=sys.stderr)
    lookup(list(wanted), cache)

    read = linked = too_long = already = 0
    review = []
    for path, (text, name) in pages.items():
        read += 1
        if "wikipedia.org" in text:
            already += 1
            continue
        found = find(name, cache)
        if not found:
            continue
        candidate, article = found
        url = url_of(article["title"])
        if len(url) > MAX_URL:
            too_long += 1
            continue
        path.write_text(add_link(text, url), encoding="utf-8")
        linked += 1
        first_sentence = re.split(r"(?<=[.!?])\s", article["extract"].strip().replace("\t", " ").replace("\n", " "))[0]
        review.append(f"{path.stem}\t{candidate}\t{article['title']}\t{first_sentence}")

    REVIEW.write_text("\n".join(review) + ("\n" if review else ""), encoding="utf-8")
    print(f"pages read: {read}, linked: {linked}, skipped for length: {too_long}, already linked: {already}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
