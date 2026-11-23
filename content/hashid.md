# hashid

Python3 program that identifies data and password hashes.

## Usage
- `hashid` : identify hashes from `stdin` (through typing, copying and pasting, or piping the hash into the program)
- `hashid hash1 hash2 ...` : identify one or more hashes
- `hashid path/to/hashes.txt` : identify hashes on a file (one hash per line)
- `hashid -e hash` : show all possible hash types (including salted hashes)
- `hashid -m -j hash` : show `hashcat`'s mode number and `john`'s format string of the hash types
- `hashid -o path/to/output.txt hash` : save output to a file instead of printing to `stdout`

## Categories
security, files, users

## Links
- More information: https://github.com/psypanda/hashID#usage
- tldr page: https://tldr.inbrowser.app/pages/common/hashid
