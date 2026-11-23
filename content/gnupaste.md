# gnupaste

This command is an alias of GNU `paste`.

## Usage
- `paste -s path/to/file` : join all the lines into a single line, using `TAB` as delimiter
- `paste -s -d delimiter path/to/file` : join all the lines into a single line, using the specified delimiter
- `paste path/to/file1 path/to/file2` : merge two files side by side, each in its column, using `TAB` as delimiter
- `paste -d delimiter path/to/file1 path/to/file2` : merge two files side by side, each in its column, using the specified delimiter
- `paste -d '\n' path/to/file1 path/to/file2` : merge two files, with lines added alternatively

## Categories
files, text, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/gnupaste
