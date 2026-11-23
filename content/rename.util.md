# rename.util

Rename multiple files. WARNING: This command will overwrite files without prompting unless the `--no-act` option is used. Note: This page refers to the command from the `util-linux` package.

## Usage
- `rename string replacement *` : rename files using simple substitutions (substitute a string with a replacement wherever found)
- `rename -vn string replacement *` : simulate running the program without doing anything
- `rename -o string replacement *` : do not overwrite existing files
- `rename .ext .bak *.ext` : change file extensions
- `rename '' 'prefix' *` : prepend a prefix to all filenames in the current directory
- `rename prefix prefix00 prefix? && rename prefix prefix0 prefix??` : rename a group of increasingly numbered files zero-padding the numbers up to 3 digits

## Categories
files, text, packages

## Links
- More information: https://manned.org/rename
- tldr page: https://tldr.inbrowser.app/pages/linux/rename.util
