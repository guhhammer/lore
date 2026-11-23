# gsha512sum

This command is an alias of GNU `sha512sum`.

## Usage
- `sha512sum path/to/file1 path/to/file2 ...` : calculate the SHA512 checksum for one or more files
- `sha512sum path/to/file1 path/to/file2 ... >path/to/file.sha512` : calculate and save the list of SHA512 checksums to a file
- `command | sha512sum` : calculate a SHA512 checksum from `stdin`
- `sha512sum -c path/to/file.sha512` : read a file of SHA512 checksums and filenames and verify all files have matching checksums
- `sha512sum -c --quiet path/to/file.sha512` : only show a message for missing files or when verification fails
- `sha512sum --ignore-missing -c --quiet path/to/file.sha512` : only show a message when verification fails, ignoring missing files
- `echo known_sha512_checksum_of_the_file path/to/file | sha512sum -c` : check a known SHA512 checksum of a file

## Categories
files, security, shell, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/gsha512sum
