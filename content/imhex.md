# imhex

Hex editor for reverse engineers and programmers.

## Usage
- `imhex path/to/file` : open a file in ImHex
- `imhex --new` : create a new empty file
- `imhex --open path/to/file --select 0xstart_offset 0xend_offset` : open a file in the currently running ImHex instance and select a range of bytes (offsets in hexadecimal)
- `imhex --file-info path/to/file` : display information about a file
- `imhex --hash md5|sha1|sha224|sha256|sha384|sha512 path/to/file` : calculate the hash of a file using a specific algorithm
- `imhex --hexdump path/to/file` : generate a hex dump of a file
- `imhex --version` : display version

## Categories
files, editors

## Links
- More information: https://docs.werwolv.net/imhex
- tldr page: https://tldr.inbrowser.app/pages/linux/imhex
