# col

Filter reverse line feeds from input.

## Usage
- `command | col` : filter reverse line feeds from input
- `command | col -x` : filter reverse line feeds and output with spaces instead of tabs
- `command | col -b` : remove backspaces, output only the last character written to each position
- `command | col -l num` : specify a buffer size with a specific number of lines
- `man ls | col -b | less` : format a manual page for viewing with `less`
- `cat path/to/input_file | col -x >output_file` : process a file with reverse line feeds and save the cleaned output

## Categories
text

## Links
- More information: https://manned.org/col
- tldr page: https://tldr.inbrowser.app/pages/linux/col
