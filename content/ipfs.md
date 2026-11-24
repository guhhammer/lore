# ipfs

Inter Planetary File System. A peer-to-peer hypermedia protocol. Aims to make the web more open.

## Usage
- `ipfs add path/to/file` : add a file from local to the filesystem, pin it, and print the relative hash
- `ipfs add -r path/to/directory` : add a directory and its files recursively from local to the filesystem and print the relative hash
- `ipfs get hash -o path/to/file` : save a remote file and give it a name but not pin it
- `ipfs pin add hash` : pin a remote file locally
- `ipfs pin ls` : display pinned files
- `ipfs pin rm hash` : unpin a file from the local storage
- `ipfs repo gc` : remove unpinned files from local storage

## Categories
files, disks, security

## Links
- More information: https://docs.ipfs.tech/reference/kubo/cli/
- tldr page: https://tldr.inbrowser.app/pages/common/ipfs
- Wikipedia: https://en.wikipedia.org/wiki/InterPlanetary_File_System
