# gnucat

This command is an alias of GNU `cat`.

## Usage
- `cat path/to/file` : print the contents of a file to `stdout`
- `cat path/to/file1 path/to/file2 ... >path/to/output_file` : concatenate several files into an output file
- `cat path/to/file1 path/to/file2 ... >>path/to/output_file` : append several files to an output file
- `cat -u /dev/tty12 >/dev/tty13` : copy the contents of a file into an output file without buffering
- `cat - >path/to/file` : write `stdin` to a file

## Categories
files, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/gnucat
