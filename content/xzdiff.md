# xzdiff

Invoke `diff` on files compressed with `xz`, `lzma`, `gzip`, `bzip2`, `lzop`, or `zstd`. All options specified are passed directly to `diff`.

## Usage
- `xzdiff path/to/file1 path/to/file2` : compare two files
- `xzdiff --side-by-side path/to/file1 path/to/file2` : compare two files, showing the differences side by side
- `xzdiff --brief path/to/file1 path/to/file2` : compare two files and report only that they differ (no details on what is different)
- `xzdiff --report-identical-files path/to/file1 path/to/file2` : compare two files and report when the files are the same
- `xzdiff --paginate path/to/file1 path/to/file2` : compare two files using paginated results

## Categories
files, archives, version-control

## Links
- More information: https://manned.org/xzdiff
- tldr page: https://tldr.inbrowser.app/pages/common/xzdiff
