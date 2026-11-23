# comp

Compare the contents of two files or sets of files. Use wildcards (*) to compare sets of files.

## Usage
- `comp` : compare files interactively
- `comp path\to\file1 path\to\file2` : compare two specified files
- `comp path\to\directory1\* path\to\directory2\*` : compare two sets of files
- `comp /d path\to\file1 path\to\file2` : display differences in decimal format
- `comp /a path\to\file1 path\to\file2` : display differences in aSCII format
- `comp /l path\to\file1 path\to\file2` : display line numbers for differences
- `comp /c path\to\file1 path\to\file2` : compare files case-insensitively
- `comp /n=5 path\to\file1 path\to\file2` : compare only the first 5 lines of each file

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, desktop, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/comp
