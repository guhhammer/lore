# gnusha224sum

This command is an alias of GNU `sha224sum`.

## Usage
- `sha224sum path/to/file1 path/to/file2 ...` : calculate the SHA224 checksum for one or more files
- `sha224sum path/to/file1 path/to/file2 ... >path/to/file.sha224` : calculate and save the list of SHA224 checksums to a file
- `command | sha224sum` : calculate a SHA224 checksum from `stdin`
- `sha224sum -c path/to/file.sha224` : read a file of SHA224 checksums and filenames and verify all files have matching checksums
- `sha224sum -c --quiet path/to/file.sha224` : only show a message for missing files or when verification fails
- `sha224sum --ignore-missing -c --quiet path/to/file.sha224` : only show a message when verification fails, ignoring missing files
- `echo known_sha224_checksum_of_the_file path/to/file | sha224sum -c` : check a known SHA224 checksum of a file

## Categories
files, security, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/gnusha224sum
