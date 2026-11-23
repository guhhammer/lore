# rmlint

Find space waste and other broken things on your filesystem.

## Usage
- `rmlint path/to/directory1 path/to/directory2 ...` : check directories for duplicated, empty, and broken files
- `rmlint -s 1MB path/to/directory // path/to/original_directory` : check for duplicates bigger than a specific size, preferably keeping files in tagged directories (after the double slash)
- `rmlint -K path/to/directory // path/to/original_directory` : check for space wasters, keeping everything in the untagged directories
- `./rmlint.sh` : delete duplicate files found by an execution of `rmlint`
- `rmlint -D path/to/directory` : find duplicate directory trees based on data, ignoring names
- `rmlint -S dl path/to/directory` : mark files at lower path depth as originals, on tie choose shorter length
- `rmlint -c sh:link -b path/to/directory` : find files with identical filename and contents, and link rather than delete the duplicates
- `rmlint path/to/backup // path/to/data -k -m` : use `data` as master directory. Find only duplicates in backup that are also in `data`. Do not delete any files in `data`

## Categories
files, disks

## Links
- More information: https://rmlint.readthedocs.io/en/latest/rmlint.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/rmlint
