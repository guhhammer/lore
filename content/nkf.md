# nkf

Network kanji filter: convert kanji code from one encoding to another.

## Usage
- `nkf -w path/to/file.txt` : convert to UTF-8 encoding
- `nkf -s path/to/file.txt` : convert to SHIFT_JIS encoding
- `nkf -w --overwrite path/to/file.txt` : convert to UTF-8 encoding and overwrite the file
- `nkf -d --overwrite path/to/file.txt` : use LF as the new line code and overwrite (UNIX type)
- `nkf -c --overwrite path/to/file.txt` : use CRLF as the new line code and overwrite (windows type)
- `nkf -m --overwrite path/to/file.txt` : decrypt mime file and overwrite

## Categories
files, network

## Links
- More information: https://manned.org/nkf
- tldr page: https://tldr.inbrowser.app/pages/common/nkf
