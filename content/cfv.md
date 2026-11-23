# cfv

Command-line File Verify, a utility to test and create checksum files.

## Usage
- `cfv -T` : test checksum files in current directory
- `cfv -Tvnu -f path/to/file` : test verbosely, rename bad, and show unverified files using a checksum file
- `cfv -C -t "sfv|sha256|md5|..." file1 file2 ...` : create a checksum file of a specific type for the selected files
- `cfv -Cr` : create separate checksum files for each recursively selected directory
- `cfv -C -f sums.sha256 -rr` : create a single `sha256` type file for all [rr]ecursively selected files
- `cfv -Czvl -p path/to/directory` : create a `gzipped` `.sfv` file verbosely, following `symlinks` and using `path/to/directory` as base path

## Categories
files, security, development

## Links
- More information: https://manned.org/cfv
- tldr page: https://tldr.inbrowser.app/pages/common/cfv
