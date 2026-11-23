# gnutee

This command is an alias of GNU `tee`.

## Usage
- `echo "example" | tee path/to/file` : copy `stdin` to each file, and also to `stdout`
- `echo "example" | tee -a path/to/file` : append to the given files, do not overwrite
- `echo "example" | tee /dev/tty | xargs printf "[%s]"` : print `stdin` to the terminal, and also pipe it into another program for further processing
- `echo "example" | tee >(xargs mkdir) >(wc -c)` : create a directory called "example", count the number of characters in "example", and write "example" to the terminal

## Categories
files, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/gnutee
