# sha256sum

Calculate SHA256 cryptographic checksums.

## Usage
- `sha256sum path/to/file1 path/to/file2 ...` : calculate the SHA256 checksum for one or more files
- `sha256sum path/to/file1 path/to/file2 ... >path/to/file.sha256` : calculate and save the list of SHA256 checksums to a file
- `command | sha256sum` : calculate a SHA256 checksum from `stdin`
- `sha256sum -c path/to/file.sha256` : read a file of SHA256 checksums and filenames and verify all files have matching checksums
- `sha256sum -c --quiet path/to/file.sha256` : only show a message for missing files or when verification fails
- `sha256sum --ignore-missing -c --quiet path/to/file.sha256` : only show a message when verification fails, ignoring missing files
- `echo known_sha256_checksum_of_the_file path/to/file | sha256sum -c` : check a known SHA256 checksum of a file

## Categories
files, security

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/sha2-utilities.html
- tldr page: https://tldr.inbrowser.app/pages/common/sha256sum
