# gnumd5sum

This command is an alias of GNU `md5sum`.

## Usage
- `md5sum path/to/file1 path/to/file2 ...` : calculate the MD5 checksum for one or more files
- `md5sum path/to/file1 path/to/file2 ... >path/to/file.md5` : calculate and save the list of MD5 checksums to a file
- `command | md5sum` : calculate an MD5 checksum from `stdin`
- `md5sum -c path/to/file.md5` : read a file of MD5 checksums and filenames and verify all files have matching checksums
- `md5sum -c --quiet path/to/file.md5` : only show a message for missing files or when verification fails
- `md5sum --ignore-missing -c --quiet path/to/file.md5` : only show a message when verification fails, ignoring missing files
- `echo known_md5_checksum_of_the_file path/to/file | md5sum -c` : check a known MD5 checksum of a file

## Categories
files, security, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/gnumd5sum
