# b3sum

Calculate BLAKE3 cryptographic checksums.

## Usage
- `b3sum path/to/file1 path/to/file2 ...` : calculate the BLAKE3 checksum for one or more files
- `b3sum path/to/file1 path/to/file2 ... >path/to/file.b3` : calculate and save the list of BLAKE3 checksums to a file
- `command | b3sum` : calculate a BLAKE3 checksum from `stdin`
- `b3sum -c path/to/file.b3` : read a file of BLAKE3 checksums and filenames and verify all files have matching checksums
- `b3sum -c --quiet path/to/file.b3` : only show a message for missing files or when verification fails
- `echo known_blake3_checksum_of_the_file path/to/file | b3sum -c` : check a known BLAKE3 checksum of a file

## Categories
files, security

## Links
- More information: https://github.com/BLAKE3-team/BLAKE3/tree/master/b3sum
- tldr page: https://tldr.inbrowser.app/pages/common/b3sum
