# rename.ul

This command is an alias of `rename.util`.

## Usage
- `rename string replacement *` : rename files using simple substitutions (substitute a string with a replacement wherever found)
- `rename -vn string replacement *` : simulate running the program without doing anything
- `rename -o string replacement *` : do not overwrite existing files
- `rename .ext .bak *.ext` : change file extensions
- `rename '' 'prefix' *` : prepend a prefix to all filenames in the current directory
- `rename prefix prefix00 prefix? && rename prefix prefix0 prefix??` : rename a group of increasingly numbered files zero-padding the numbers up to 3 digits

## Categories
files, text, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/rename.ul
