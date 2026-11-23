# less-than-more-than

Open a file descriptor for read and write.

## Usage
- `exec 3<>path/to/file` : open a file in a file descriptor for read and write
- `exec 3<>/dev/tcp/remote_host/port_number` : open a file descriptor to a remote connection
- `exec 3<>-` : close a file descriptor

## Categories
files

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/less-than-more-than
