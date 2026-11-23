# sudoreplay

Replay sudo session logs.

## Usage
- `sudoreplay [-FhnRS] [-d dir] [-f filter] [-m num] [-s num] ID[@offset]` : the general form, as the manual gives it
- `sudoreplay [-h] [-d dir] -l [search expression]` : another form of the same command
- `sudoreplay -d dir` : store session logs in dir instead of the default, /var/log/sudo-io
- `sudoreplay -f filter` : select  which  I/O  type(s) to display
- `sudoreplay -h` : display a short help message to the standard output and exit
- `sudoreplay -S` : wait  while  the command was suspended

## Categories
users

## Links
- Documentation: https://www.sudo.ws/about/contributors/
