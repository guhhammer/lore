"""Step 6: tldr's pages for other systems (Windows, the BSDs, SunOS,
Android) → content/, for names no page uses yet. lore lists none of these
commands (they do not run where lore does) but opens them with `lore NAME`;
each page says which system it is for."""

from common import CONTENT, OTHER_SYSTEMS, Page, file_name, read_tldr, tldr_files, tldr_links


def note(platform: str) -> str:
    if platform == "android":
        return "An Android command: run it on a device or emulator, usually through `adb shell`."
    system = OTHER_SYSTEMS[platform]
    return f"A {system} command: this page is for reference, it does not run on Linux or macOS."


def main() -> None:
    made = dict.fromkeys(OTHER_SYSTEMS, 0)
    for platform, path in tldr_files(platforms=OTHER_SYSTEMS):
        name = path.stem
        if (CONTENT / file_name(name)).exists():
            continue
        tldr = read_tldr(path)
        if not tldr.summary:
            continue
        title = tldr.title.lower() if tldr.title.replace(" ", "-").lower() == name else name
        Page(title, tldr.summary, tldr.examples, [note(platform)],
             tldr_links(tldr, platform, name)).save(CONTENT, name)
        made[platform] += 1
    print(", ".join(f"{OTHER_SYSTEMS[p]} {n}" for p, n in made.items()))


if __name__ == "__main__":
    main()
