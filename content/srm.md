# srm

Securely remove files or directories. Overwrites the existing data one or multiple times. Drop in replacement for rm.

## Usage
- `srm -s path/to/file` : remove a file after a single-pass overwriting with random data
- `srm -m path/to/file` : remove a file after seven passes of overwriting with random data
- `srm -r -s path/to/directory` : recursively remove a directory and its contents overwriting each file with a single-pass of random data
- `srm -i \*` : prompt before every removal

## Categories
danger, files

## Links
- More information: https://srm.sourceforge.net/srm.html
- tldr page: https://tldr.inbrowser.app/pages/common/srm
- Wikipedia: https://en.wikipedia.org/wiki/Srm_(Unix)
