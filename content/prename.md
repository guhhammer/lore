# prename

This command is an alias of `rename`.

## Usage
- `rename 's/from/to/' *.txt` : replace `from` with `to` in the filenames of the specified files
- `rename -n 's/from/to/' *.txt` : simulate running the program without doing anything
- `rename 's/\.old$/\.new/' *.txt` : change the extension
- `rename -f 'y/A-Z/a-z/' *.txt` : change to lowercase (use `-f` in case-insensitive filesystems)
- `rename -f 's/\b(\w)/\U$1/g' *.txt` : capitalize first letter of every word in the name
- `rename 's/\s+/_/g' *.txt` : replace spaces with underscores

## Categories
files, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/prename
