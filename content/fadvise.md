# fadvise

Control Linux file caching behavior.

## Usage
- `fadvise -a willneeded path/to/file_or_directory` : preload a file or directory into cache
- `fadvise path/to/file` : suggest dropping a file from cache
- `fadvise -h` : display help

## See also
- fincore

## Categories
files

## Links
- More information: https://manned.org/fadvise
- tldr page: https://tldr.inbrowser.app/pages/linux/fadvise
