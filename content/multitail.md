# multitail

Extension of tail.

## Usage
- `multitail -Q 1 'pattern'` : tail all files matching a pattern in a single stream
- `multitail -Q 1 'path/to/directory/*'` : tail all files in a directory in a single stream
- `multitail -Q pattern` : automatically add new files to a window
- `multitail -s 2 -sn 1,3 path/to/mergefile -I path/to/file1 path/to/file2 path/to/file3 path/to/file4` : show 5 logfiles while merging 2 and put them in 2 columns with only one in the left column

## Categories
files, text

## Links
- More information: https://manned.org/multitail
- tldr page: https://tldr.inbrowser.app/pages/common/multitail
