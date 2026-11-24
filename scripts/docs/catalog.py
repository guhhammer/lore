"""The docs topic's catalogue: every documentation set DevDocs
(https://devdocs.io) offers, written to devdocs/catalog.tsv, built into
lore. lore downloads a set from DevDocs only when the reader asks for it
(`d` in the docs topic).

Each set is ranked by how much people ask about it: the Stack Overflow
tags with the most questions (its public API), matched to the set's name.
Google's search counts are not public; Stack Overflow's are the closest
public measure of what people look up.

Standard library only. From the repository root:

    python3 scripts/docs/catalog.py
"""

from __future__ import annotations

import datetime
import html
import json
import re
import time
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "devdocs" / "catalog.tsv"
CATALOGUE = "https://devdocs.io/docs.json"
TAGS = "https://api.stackexchange.com/2.3/tags?order=desc&sort=popular&site=stackoverflow&pagesize=100&page={page}"
AGENT = "lore-docs-catalog/1.0"
TAG_PAGES = 10  # the 1,000 most asked-about tags

# a set's base name -> its Stack Overflow tag, where they differ
TAG_OF = {
    "react": "reactjs", "node": "node.js", "openjdk": "java", "cpp": "c++", "vue": "vue.js",
    "rails": "ruby-on-rails", "nextjs": "next.js", "tailwindcss": "tailwind-css",
    "jest": "jestjs", "d3": "d3.js", "three_js": "three.js", "scikit_learn": "scikit-learn",
    "express": "express", "dom": "dom", "angularjs": "angularjs", "backbone": "backbone.js",
    "ember": "ember.js", "moment": "momentjs", "socketio": "socket.io", "knockout": "knockout.js",
    "chartjs": "chart.js", "electron": "electron", "nginx": "nginx", "bash": "bash",
}


def get(url: str) -> bytes:
    request = urllib.request.Request(url, headers={"User-Agent": AGENT, "Accept-Encoding": "identity"})
    with urllib.request.urlopen(request, timeout=60) as response:
        return response.read()


def asked() -> dict[str, int]:
    """Stack Overflow tag -> its rank (1: the most asked about)."""
    counts: dict[str, int] = {}
    for page in range(1, TAG_PAGES + 1):
        for item in json.loads(get(TAGS.format(page=page)))["items"]:
            counts[item["name"]] = item["count"]
        time.sleep(1)
    # each page comes sorted by name, not by count: ranked here
    ordered = sorted(counts, key=lambda tag: -counts[tag])
    return {tag: rank for rank, tag in enumerate(ordered, start=1)}


def plain(text: str) -> str:
    """HTML to one line of text."""
    return " ".join(html.unescape(re.sub(r"<[^>]+>", " ", text or "")).split())


def main() -> None:
    sets = json.loads(get(CATALOGUE))
    ranks = asked()
    OUT.parent.mkdir(parents=True, exist_ok=True)
    with OUT.open("w", encoding="utf-8") as out:
        out.write(f"# lore docs catalogue, {datetime.date.today().isoformat()}: scripts/docs/catalog.py\n")
        out.write("# slug\tname\tversion\trelease\tbytes\tmtime\tstack overflow rank (0: none)\thome\tattribution\n")
        for docset in sets:
            base = docset["slug"].split("~")[0]
            tags = [TAG_OF.get(base, base), base.replace("_", "-"), base + ".js", base + "js"]
            rank = min((ranks[t] for t in tags if t in ranks), default=0)
            fields = [
                docset["slug"], docset["name"], docset.get("version") or "", docset.get("release") or "",
                str(docset.get("db_size") or 0), str(docset.get("mtime") or 0), str(rank),
                (docset.get("links") or {}).get("home", ""), plain(docset.get("attribution", "")),
            ]
            out.write("\t".join(f.replace("\t", " ").replace("\n", " ") for f in fields) + "\n")
    print(f"{len(sets)} sets -> {OUT}")


if __name__ == "__main__":
    main()
