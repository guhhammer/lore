# findstr

Find specified text within one or more files.

## Usage
- `findstr "string1 string2 ..." *` : find one or more strings in all files
- `dir | findstr "string1 string2 ..."` : find one or more strings in a piped command's output
- `findstr /s "string1 string2 ..." *` : find one or more strings in all files recursively
- `findstr /i "string1 string2 ..." *` : find strings using a case-insensitive search
- `findstr /r "regex" *` : find strings in all files using `regex`
- `findstr /c:"string1 string2 ..." *.txt` : find a literal string (containing spaces) in all text files
- `findstr /n "string1 string2 ..." *` : display the line number before each matching line
- `findstr /m "string1 string2 ..." *` : display only the filenames that contain a match

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
text, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/findstr
