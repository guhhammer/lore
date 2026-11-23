# cvfsck

Check and repair an Xsan filesystem volume. Part of the Xsan filesystem utilities on macOS.

## Usage
- `sudo cvfsck /Volumes/XsanVolume` : check an Xsan volume for metadata corruption (read-only mode)
- `sudo cvfsck -w /Volumes/XsanVolume` : repair a corrupted Xsan volume (make modifications to fix problems)
- `sudo cvfsck -l /Volumes/XsanVolume` : log problems to the system log (used mainly during automatic startup checks)
- `sudo cvfsck -f /Volumes/XsanVolume` : report free-space fragmentation on an Xsan volume

## Categories
disks, users, macos

## Links
- More information: https://www.manpagez.com/man/1/cvfsck/
- tldr page: https://tldr.inbrowser.app/pages/osx/cvfsck
