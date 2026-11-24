# free disk space

Find what fills a disk, then clear what is safe to clear: old package
files, logs, caches.

## Steps
- `df -h` : see how full each disk is, and which one is short of space
- `du -xh -d 1 / 2>/dev/null | sort -h` : the biggest folders at the top of that disk, biggest last
- `ncdu -x /` : walk through the folders by size and delete from there, if ncdu is installed
- `sudo journalctl --vacuum-size=200M` : shrink the system's logs to 200 MB
- `sudo apt clean` : remove the downloaded package files (Debian, Ubuntu)
- `sudo apt autoremove` : remove the packages nothing needs any more (Debian, Ubuntu)
- `rm -rf ~/.cache/thumbnails` : delete the cached thumbnails, made again when needed

## Notes
- Fedora and RHEL: `sudo dnf clean all` and `sudo dnf autoremove`. Arch: `sudo pacman -Sc`.
- `-x` keeps `du` and `ncdu` on that one disk, so other disks and network folders are not counted.
- macOS: the same `df -h` and `du -xh -d 1 /`; Homebrew's downloads go with `brew cleanup`.

## See also
- df
- du
- ncdu
- journalctl

## Categories
disks files system
