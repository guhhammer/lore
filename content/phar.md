# phar

Create, update, or extract PHP archives (PHAR).

## Usage
- `phar add -f path/to/phar_file path/to/file_or_directory1 path/to/file_or_directory2 ...` : add one or more files or directories to a Phar file
- `phar list -f path/to/phar_file` : display the contents of a Phar file
- `phar delete -f path/to/phar_file -e path/to/file_or_directory` : delete the specified file or directory from a Phar file
- `phar compress -f path/to/phar_file -c algorithm` : compress or uncompress files and directories in a Phar file
- `phar info -f path/to/phar_file` : get information about a Phar file
- `phar sign -f path/to/phar_file -h algorithm` : sign a Phar file with a specific hash algorithm
- `phar sign -f path/to/phar_file -h openssl -y path/to/private_key` : sign a Phar file with an OpenSSL private key
- `phar help` : display help and available hashing/compression algorithms

## Categories
files, archives

## Links
- More information: https://manned.org/phar
- tldr page: https://tldr.inbrowser.app/pages/linux/phar
