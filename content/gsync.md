# gsync

This command is an alias of GNU `sync`.

## Usage
- `sync` : flush all pending write operations on all disks
- `sync path/to/file` : flush all pending write operations on a single file to disk
- `sync; echo 3 | sudo tee /proc/sys/vm/drop_caches` : flush writes and drop filesystem caches

## Categories
disks, files, shell, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/gsync
