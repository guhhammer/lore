# lsyncd

Watch files and directories and run `rsync` when they change. It is often used to keep two directories on separate systems in sync, ensuring that changes made in one directory are immediately mirrored to the other.

## Usage
- `lsyncd -rsync path/to/source host::share_name` : watch the source for changes and run `rsync` to synchronize files to the destination on every change
- `lsyncd -rsyncssh path/to/source host path/to/destination` : use SSH instead of `rsyncd` shares

## Categories
files, network

## Links
- More information: https://github.com/lsyncd/lsyncd/blob/master/docs/manpage/lsyncd.1.txt
- tldr page: https://tldr.inbrowser.app/pages/common/lsyncd
