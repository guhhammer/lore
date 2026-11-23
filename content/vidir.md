# vidir

Edit directories in a text editor.

## Usage
- `vidir path/to/directory1 path/to/directory2 ...` : edit the contents of the specified directories
- `vidir -v path/to/directory1 path/to/directory2 ...` : display each action taken by the program
- `vidir` : edit the contents of current directory
- `EDITOR=vim vidir path/to/directory1 path/to/directory2 ...` : use the specified text editor
- `command | vidir -` : read a list of files to edit from `stdin`

## Categories
files, editors, text

## Links
- More information: https://manned.org/vidir
- tldr page: https://tldr.inbrowser.app/pages/common/vidir
