# isutf8

Check whether text files contain valid UTF-8.

## Usage
- `isutf8 path/to/file1 path/to/file2 ...` : check whether the specified files contain valid UTF-8
- `isutf8 -v path/to/file1 path/to/file2 ...` : print errors using multiple lines
- `isutf8 -q path/to/file1 path/to/file2 ...` : do not print anything to `stdout`, indicate the result merely with the exit code
- `isutf8 -l path/to/file1 path/to/file2 ...` : only print the names of the files containing invalid UTF-8
- `isutf8 -i path/to/file1 path/to/file2 ...` : only print the names of the files containing valid UTF-8 (the inverse of `--list`)

## Categories
files, documents, text

## Links
- More information: https://manned.org/isutf8
- tldr page: https://tldr.inbrowser.app/pages/common/isutf8
