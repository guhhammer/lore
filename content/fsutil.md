# fsutil

Display information about filesystem volumes.

## Usage
- `fsutil volume list` : display a list of volumes
- `fsutil fsInfo volumeInfo drive_letter|path/to/volume` : display information about a volume's filesystem
- `fsutil repair state` : display the current state of the filesystem auto-repair for all volumes
- `fsutil dirty query` : display the dirty bit state of all volumes
- `fsutil dirty set drive_letter|path/to/volume` : set the dirty bit state of a volume

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
disks, desktop, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/fsutil
