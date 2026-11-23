# gcomm

This command is an alias of GNU `comm`.

## Usage
- `comm file1 file2` : produce three tab-separated columns: lines only in first file, lines only in second file, and common lines
- `comm -12 file1 file2` : print only lines common to both files
- `cat file1 | comm -12 - file2` : print only lines common to both files, reading one file from `stdin`
- `comm -23 file1 file2 >file1_only` : get lines only found in first file, saving the result to a third file
- `comm -13 <(sort file1) <(sort file2)` : print lines only found in second file, when the files aren't sorted

## Categories
text, files, documents, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/gcomm
