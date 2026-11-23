# lsfd

List open files and the corresponding processes in Linux.

## Usage
- `lsfd` : list all open file descriptors
- `lsfd -Q 'PID == process_id'` : list all files kept open by a specific program
- `lsfd -Q "NAME == 'path/to/file'"` : check what program has a specific file open
- `lsfd -i4|-i6` : list open IPv4 or IPv6 sockets
- `lsfd -h` : display help

## Categories
files, processes

## Links
- More information: https://manned.org/lsfd
- tldr page: https://tldr.inbrowser.app/pages/linux/lsfd
