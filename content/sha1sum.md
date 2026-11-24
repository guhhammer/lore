# sha1sum

Calculate SHA1 cryptographic checksums.

## Usage
- `sha1sum path/to/file1 path/to/file2 ...` : calculate the SHA1 checksum for one or more files
- `sha1sum path/to/file1 path/to/file2 ... >path/to/file.sha1` : calculate and save the list of SHA1 checksums to a file
- `command | sha1sum` : calculate a SHA1 checksum from `stdin`
- `sha1sum -c path/to/file.sha1` : read a file of SHA1 checksums and filenames and verify all files have matching checksums
- `sha1sum -c --quiet path/to/file.sha1` : only show a message for missing files or when verification fails
- `sha1sum --ignore-missing -c --quiet path/to/file.sha1` : only show a message when verification fails, ignoring missing files
- `echo known_sha1_checksum_of_the_file path/to/file | sha1sum -c` : check a known SHA1 checksum of a file

## Categories
files, security

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/sha1sum
- Wikipedia: https://en.wikipedia.org/wiki/Sha1sum
