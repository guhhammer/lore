# enca

Detect and convert the encoding of text files.

## Usage
- `enca path/to/file1 path/to/file2 ...` : detect file(s) encoding according to the system's locale
- `enca -L language path/to/file1 path/to/file2 ...` : detect file(s) encoding specifying a language in the POSIX/C locale format (e.g. zh_CN, en_US)
- `enca -L language -x to_encoding path/to/file1 path/to/file2 ...` : convert file(s) to a specific encoding
- `enca <original_file -L language -x to_encoding >new_file` : create a copy of an existing file using a different encoding

## Categories
files, text

## Links
- More information: https://manned.org/enca
- tldr page: https://tldr.inbrowser.app/pages/common/enca
