# sync

Flush all pending write operations to the appropriate disks.

## Usage
- `sync` : flush all pending write operations on all disks
- `sync path/to/file` : flush all pending write operations on a single file to disk
- `sync; echo 3 | sudo tee /proc/sys/vm/drop_caches` : flush writes and drop filesystem caches

## Categories
disks, files

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/sync-invocation.html
- tldr page: https://tldr.inbrowser.app/pages/linux/sync
- Wikipedia: https://en.wikipedia.org/wiki/Sync_(Unix)
