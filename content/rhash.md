# rhash

Calculate or check common message digests.

## Usage
- `rhash path/to/file` : calculate default CRC32 digests of a file
- `rhash --sha1 --recursive path/to/folder >path/to/output.sfv` : recursively process a directory to generate an SFV file using SHA1
- `rhash --check path/to/file.sfv` : verify the integrity of files based on an SFV file
- `rhash --sha3-256 --message 'message'` : calculate the SHA3 digest of a text message
- `rhash --base64 --bsd path/to/file` : calculate CRC32 digest of a file and output digest encoded in Base64 using BSD format
- `rhash --printf '%p\t%s\t%{mtime}\t%m\n' path/to/file` : use custom output template

## Categories
files

## Links
- More information: https://rhash.sourceforge.io/manpage.php
- tldr page: https://tldr.inbrowser.app/pages/common/rhash
