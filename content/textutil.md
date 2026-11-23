# textutil

Manipulate text files of various formats.

## Usage
- `textutil -info path/to/file.rtf` : display information about `file.rtf`
- `textutil -convert html path/to/file.rtf` : convert `file.rtf` into `file.html`
- `textutil path/to/file.rtf -convert txt` : convert rich text to normal text
- `textutil -convert rtf -font Times -fontsize 10 path/to/file.txt` : convert `file.txt` into `file.rtf`, using Times 10 for the font
- `textutil -cat html -title "Several Files" -output path/to/index.html *.rtf` : load all RTF files in the current directory, concatenate their contents, and write the result out as `index.html` with the HTML title set to "Several Files"

## Categories
files, text, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/textutil.1.html
- tldr page: https://tldr.inbrowser.app/pages/osx/textutil
