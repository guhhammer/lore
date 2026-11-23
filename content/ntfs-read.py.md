# ntfs-read.py

A read-only NTFS explorer for accessing and extracting files from NTFS volumes. Part of the Impacket suite.

## Usage
- `ntfs-read.py volume` : open an NTFS volume for exploration (e.g., `C:\.\\` or `/dev/disk1s1`)
- `ntfs-read.py -extract \windows\system32\config\sam volume` : extract a specific file from an NTFS volume (e.g., `\windows\system32\config\sam`)
- `ntfs-read.py -debug volume` : enable debug output
- `ntfs-read.py --help` : display help

## Categories
disks

## Links
- More information: https://github.com/fortra/impacket
- tldr page: https://tldr.inbrowser.app/pages/common/ntfs-read.py
