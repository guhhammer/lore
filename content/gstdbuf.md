# gstdbuf

This command is an alias of GNU `stdbuf`.

## Usage
- `stdbuf -i 512K command` : change `stdin` buffer size to 512 KiB
- `stdbuf -o L command` : change `stdout` buffer to line-buffered
- `stdbuf -e 0 command` : change `stderr` buffer to unbuffered

## Categories
shell, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/gstdbuf
