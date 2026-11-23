# while

Simple shell loop that repeats while the return value remains zero.

## Usage
- `while read line; do echo "$line"; done` : read `stdin` and perform an action on every line
- `while :; do command; sleep 1; done` : execute a command forever once every second
- `while command; do :; done` : execute a command until it fails

## Categories
shell, text

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-while
- tldr page: https://tldr.inbrowser.app/pages/common/while
