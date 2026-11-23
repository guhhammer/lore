# namei

Follow a pathname (which can be a symbolic link) until a terminal point is found (a file/directory/char device etc.). This program is useful for finding "too many levels of symbolic links" problems.

## Usage
- `namei path/to/a path/to/b path/to/c` : resolve the pathnames specified as the argument parameters
- `namei -l path/to/a path/to/b path/to/c` : display the results in a long-listing format
- `namei -m path/to/a path/to/b path/to/c` : show the mode bits of each file type in the style of `ls`
- `namei -o path/to/a path/to/b path/to/c` : show owner and group name of each file
- `namei -n path/to/a path/to/b path/to/c` : don't follow symlinks while resolving

## Categories
files, hardware, shell

## Links
- More information: https://manned.org/namei
- tldr page: https://tldr.inbrowser.app/pages/linux/namei
