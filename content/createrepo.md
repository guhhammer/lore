# createrepo

Initialize an RPM repository in a directory, including all XML and SQLite files.

## Usage
- `createrepo path/to/directory` : initialize a basic repository in a directory
- `createrepo -v -x test_*.rpm path/to/directory` : initialize a repository, exclude test RPMs, and display verbose logs
- `createrepo -S -s sha1 path/to/directory` : initialize a repository, using SHA1 as the checksum algorithm, and ignoring symbolic links

## Categories
files, packages, databases

## Links
- More information: https://manned.org/createrepo
- tldr page: https://tldr.inbrowser.app/pages/linux/createrepo
