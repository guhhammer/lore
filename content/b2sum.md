# b2sum

Calculate BLAKE2 cryptographic checksums.

## Usage
- `b2sum path/to/file1 path/to/file2 ...` : calculate the BLAKE2 checksum for one or more files
- `b2sum path/to/file1 path/to/file2 ... >path/to/file.b2` : calculate and save the list of BLAKE2 checksums to a file
- `command | b2sum` : calculate a BLAKE2 checksum from `stdin`
- `b2sum -c path/to/file.b2` : read a file of BLAKE2 checksums and filenames and verify all files have matching checksums
- `b2sum -c --quiet path/to/file.b2` : only show a message for missing files or when verification fails
- `b2sum --ignore-missing -c --quiet path/to/file.b2` : only show a message when verification fails, ignoring missing files
- `echo known_blake2_checksum_of_the_file path/to/file | b2sum -c` : check a known BLAKE2 checksum of a file

## Categories
files, security

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/b2sum
