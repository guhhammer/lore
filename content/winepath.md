# winepath

Convert file paths between Windows and Unix formats for a Wine prefix.

## Usage
- `winepath -u 'C:\path\to\file'` : convert a Windows path to a Unix path
- `winepath -w path/to/file` : convert a Unix path to a long Windows path
- `winepath -l 'C:\PROGRA~1'` : convert a short (8.3) Windows path of an existing file to its long form
- `winepath -s 'C:\Program Files'` : convert a long Windows path of an existing file to its short (8.3) form
- `winepath -u -0 path1 path2 ...` : separate output paths with a null character instead of a newline
- `winepath -h` : display help

## Categories
files

## Links
- More information: https://manned.org/winepath
- tldr page: https://tldr.inbrowser.app/pages/linux/winepath
