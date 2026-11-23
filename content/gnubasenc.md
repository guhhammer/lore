# gnubasenc

This command is an alias of GNU `basenc`.

## Usage
- `basenc --base64 path/to/file` : encode a file with Base64 encoding
- `basenc -d --base64 path/to/file` : decode a file with Base64 encoding
- `command | basenc --base32 -w 42` : encode from `stdin` with Base32 encoding with 42 columns
- `command | basenc --base32` : encode from `stdin` with Base32 encoding

## Categories
files, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/gnubasenc
