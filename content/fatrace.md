# fatrace

Report file access events.

## Usage
- `sudo fatrace` : print file access events in all mounted filesystems to `stdout`
- `sudo fatrace -C program_name` : limit output to a program with a specific name
- `sudo fatrace -c` : print file access events on the mount of the current directory to `stdout`
- `sudo fatrace -t` : add timestamps to the printout

## See also
- inotifywait

## Categories
files, users

## Links
- More information: https://manned.org/fatrace
- tldr page: https://tldr.inbrowser.app/pages/linux/fatrace
