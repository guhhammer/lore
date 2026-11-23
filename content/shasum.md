# shasum

Calculate SHA cryptographic checksums.

## Usage
- `shasum path/to/file1 path/to/file2 ...` : calculate the SHA1 checksum for one or more files
- `shasum --algorithm 1|224|256|384|512|512224|512256 path/to/file1 path/to/file2 ...` : calculate the SHA checksum for one or more files with the specified algorithm
- `command | shasum` : calculate a SHA1 checksum from `stdin`
- `shasum --algorithm 256 path/to/file1 path/to/file2 ... >path/to/file.sha256` : calculate and save the list of SHA256 checksums to a file
- `shasum -c path/to/file` : read a file of SHA checksums and filenames and verify all files have matching checksums (the algorithm will be automatically detected)
- `shasum -c --quiet path/to/file` : only show a message for missing files or when verification fails
- `shasum --ignore-missing -c --quiet path/to/file` : only show a message when verification fails, ignoring missing files
- `echo 'known_sha_checksum_of_the_file  path/to/file' | shasum -c` : check a known SHA checksum of a file (Note: two spaces separating checksum and path)

## Categories
files, security

## Links
- More information: https://manned.org/shasum
- tldr page: https://tldr.inbrowser.app/pages/common/shasum
