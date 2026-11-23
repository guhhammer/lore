# sdiff

Compare the differences between and optionally merge 2 files.

## Usage
- `sdiff path/to/file1 path/to/file2` : compare 2 files
- `sdiff -W path/to/file1 path/to/file2` : compare 2 files, ignoring all tabs and whitespace
- `sdiff -Z path/to/file1 path/to/file2` : compare 2 files, ignoring whitespace at the end of lines
- `sdiff -i path/to/file1 path/to/file2` : compare 2 files in a case-insensitive manner
- `sdiff -o path/to/merged_file path/to/file1 path/to/file2` : compare and then merge, writing the output to a new file

## Categories
files, version-control

## Links
- More information: https://manned.org/sdiff
- tldr page: https://tldr.inbrowser.app/pages/common/sdiff
