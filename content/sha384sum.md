# sha384sum

Calculate SHA384 cryptographic checksums.

## Usage
- `sha384sum path/to/file1 path/to/file2 ...` : calculate the SHA384 checksum for one or more files
- `sha384sum path/to/file1 path/to/file2 ... >path/to/file.sha384` : calculate and save the list of SHA384 checksums to a file
- `command | sha384sum` : calculate a SHA384 checksum from `stdin`
- `sha384sum -c path/to/file.sha384` : read a file of SHA384 checksums and filenames and verify all files have matching checksums
- `sha384sum -c --quiet path/to/file.sha384` : only show a message for missing files or when verification fails
- `sha384sum --ignore-missing -c --quiet path/to/file.sha384` : only show a message when verification fails, ignoring missing files
- `echo known_sha384_checksum_of_the_file path/to/file | sha384sum -c` : check a known SHA384 checksum of a file

## Categories
files, security

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/sha2-utilities.html
- tldr page: https://tldr.inbrowser.app/pages/common/sha384sum
