# ln

Create links to files and directories.

## Usage
- `ln -s /path/to/file_or_directory path/to/symlink` : create a symbolic link to a file or directory
- `ln -s path/to/file_or_directory path/to/symlink` : create a symbolic link relative to where the link is located
- `ln -sf /path/to/new_file path/to/symlink` : overwrite an existing symbolic link to point to a different file
- `ln /path/to/file path/to/hardlink` : create a hard link to a file

## Categories
files

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/ln-invocation.html
- tldr page: https://tldr.inbrowser.app/pages/common/ln
- Wikipedia: https://en.wikipedia.org/wiki/Ln_(Unix)
