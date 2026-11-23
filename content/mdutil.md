# mdutil

Manage the metadata stores used by Spotlight for indexing.

## Usage
- `mdutil -s /` : show the indexing status of the startup volume
- `mdutil -i on|off path/to/volume` : turn on/off the Spotlight indexing for a given volume
- `mdutil -a -i on|off` : turn on/off indexing for all volumes
- `mdutil -E path/to/volume` : erase the metadata stores and restart the indexing process

## Categories
disks, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/mdutil.1.html
- tldr page: https://tldr.inbrowser.app/pages/osx/mdutil
