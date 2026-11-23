# gnusplit

This command is an alias of GNU `split`.

## Usage
- `split -l 10 path/to/file` : split a file, each split having 10 lines (except the last split)
- `split -n 5 path/to/file` : split a file into 5 files. File is split such that each split has same size (except the last split)
- `split -b 512 path/to/file` : split a file with 512 bytes in each split (except the last split; use 512k for kilobytes and 512m for megabytes)
- `split -C 512 path/to/file` : split a file with at most 512 bytes in each split without breaking lines
- `gzip -cd path/to/compressed_file.gz | split -l 1000 - path/to/output` : split into multiple files from `stdin`

## Categories
files, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/gnusplit
