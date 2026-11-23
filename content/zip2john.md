# zip2john

Extract password hashes from Zip archives for use with John the Ripper password cracker. This is a utility tool usually installed as part of the John the Ripper installation.

## Usage
- `zip2john path/to/file.zip` : extract the password hash from an archive, listing all files in the archive
- `zip2john -o path/to/compressed_file path/to/file.zip` : extract the password hash using only a specific compressed file
- `zip2john -o path/to/compressed_file path/to/file.zip >file.hash` : extract the password hash from a compressed file to a specific file (for use with John the Ripper)

## Categories
archives, files, users

## Links
- More information: https://www.openwall.com/john/
- tldr page: https://tldr.inbrowser.app/pages/common/zip2john
