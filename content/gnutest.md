# gnutest

This command is an alias of GNU `test`.

## Usage
- `test "$MY_VAR" = "/bin/zsh"` : test if a given variable is equal to a given string
- `test -z "$GIT_BRANCH"` : test if a given variable is empty (zero length)
- `test -f "path/to/file"` : test if a file exists
- `test ! -d "path/to/directory"` : test if a directory does not exist
- `test condition && echo "true" || echo "false"` : if A is true, then do B, or C in the case of an error (notice that C may run even if A fails)
- `if test -f "path/to/file"; then echo "File exists"; else echo "File does not exist"; fi` : use `test` in a conditional statement
- `help test` : display help

## Categories
development, files, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/gnutest
