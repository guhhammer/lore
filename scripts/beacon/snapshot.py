"""The beacon's data: how many people use each command, from two public
sources, written to beacon/beacon.tsv (built into lore, and published each
week by .github/workflows/content.yml).

- Debian popcon (https://popcon.debian.org/by_inst): per package, the
  machines that use it regularly ("vote"). Packages on nearly every machine
  (the base system) are left out: nobody chose them.
- Homebrew analytics (https://formulae.brew.sh/api/analytics/): installs
  asked for by name (not pulled in by another formula), per formula, over
  the last 365 and 30 days; the two give the trend.

Packages become commands through Debian's Contents file (which package
installs /usr/bin/NAME). Only commands with a page in content/ are kept, as
the beacon shows pages. lore ranks them itself (src/info/beacon.rs); this
only gathers the numbers.

Standard library only. Run from the repository root:

    python3 scripts/beacon/snapshot.py            # writes beacon/beacon.tsv
    python3 scripts/beacon/snapshot.py --out FILE
"""

from __future__ import annotations

import argparse
import datetime
import gzip
import io
import json
import re
import urllib.request
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CONTENT = ROOT / "content"
OUT = ROOT / "beacon" / "beacon.tsv"

POPCON = "https://popcon.debian.org/by_inst"
CONTENTS = "https://deb.debian.org/debian/dists/stable/main/Contents-amd64.gz"
BREW = "https://formulae.brew.sh/api/analytics/install-on-request/{days}d.json"
AGENT = "lore-beacon (https://github.com/guhhammer/lore)"

# a package on this share of the most installed one's machines is the base
# system (src/compile_config.rs has the same value for lore's own fallback)
BASE_SHARE = 0.9
# a package gives the command named like it; without one, all of its
# commands if it has at most this many (ripgrep: rg), else none
MAX_COMMANDS = 3
# how many commands the file keeps, most used first
KEEP = 1500
BIN_DIRS = ("usr/bin/", "usr/sbin/", "bin/", "sbin/")


def get(url: str) -> bytes:
    request = urllib.request.Request(url, headers={"User-Agent": AGENT})
    with urllib.request.urlopen(request, timeout=120) as response:
        return response.read()


def popcon() -> dict[str, int]:
    """package → machines that use it regularly; base packages left out."""
    votes, top = {}, 0
    for line in get(POPCON).decode("utf-8", "replace").splitlines():
        if line.startswith("#"):
            continue
        fields = line.split()
        if len(fields) < 4 or not fields[0].isdigit():
            continue
        name, inst, vote = fields[1], int(fields[2]), int(fields[3])
        top = max(top, inst)
        if inst < top * BASE_SHARE:
            votes[name] = vote
    return votes


def contents() -> dict[str, list[str]]:
    """package → the commands it installs."""
    commands = defaultdict(set)
    text = gzip.GzipFile(fileobj=io.BytesIO(get(CONTENTS))).read().decode("utf-8", "replace")
    for line in text.splitlines():
        path, _, owners = line.rpartition(" ")
        path = path.strip()
        if not path.startswith(BIN_DIRS):
            continue
        command = path.rsplit("/", 1)[-1]
        for owner in owners.split(","):
            commands[owner.rsplit("/", 1)[-1]].add(command)
    return {package: sorted(names) for package, names in commands.items()}


def brew(days: int) -> dict[str, int]:
    """formula (its version suffix dropped: python@3.13 → python) → installs."""
    counts: dict[str, int] = defaultdict(int)
    for item in json.loads(get(BREW.format(days=days)))["items"]:
        name = item["formula"].split("@")[0]
        counts[name] += int(str(item["count"]).replace(",", ""))
    return counts


def chosen(package: str, installs: list[str]) -> list[str]:
    """The commands a package counts for: its namesake, else all of a few."""
    if package in installs:
        return [package]
    return installs if len(installs) <= MAX_COMMANDS else []


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--out", type=Path, default=OUT)
    out = parser.parse_args().out

    pages = {p.stem for p in CONTENT.glob("*.md")}
    by_package = contents()
    debian, year, month = defaultdict(int), defaultdict(int), defaultdict(int)
    for package, vote in popcon().items():
        for command in chosen(package, by_package.get(package, [package])):
            debian[command] = max(debian[command], vote)
    for counts, into in ((brew(365), year), (brew(30), month)):
        for formula, count in counts.items():
            for command in chosen(formula, by_package.get(formula, [formula])):
                into[command] = max(into[command], count)

    names = [n for n in set(debian) | set(year) if n in pages and re.fullmatch(r"[\w.+-]+", n)]
    total_debian = sum(debian[n] for n in names) or 1
    total_year = sum(year[n] for n in names) or 1
    names.sort(key=lambda n: (-(debian[n] / total_debian + year[n] / total_year), n))

    out.parent.mkdir(parents=True, exist_ok=True)
    with out.open("w", encoding="utf-8") as file:
        file.write(f"# lore beacon, {datetime.date.today().isoformat()}: scripts/beacon/snapshot.py\n")
        file.write("# command\tdebian popcon votes\thomebrew installs, 365 days\thomebrew installs, 30 days\n")
        for name in names[:KEEP]:
            file.write(f"{name}\t{debian[name]}\t{year[name]}\t{month[name]}\n")
    print(f"{min(len(names), KEEP)} commands -> {out}")


if __name__ == "__main__":
    main()
