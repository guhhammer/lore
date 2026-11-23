# chattr

Change attributes of files or directories.

## Usage
- `sudo chattr +i path/to/file_or_directory` : make a file or directory immutable to changes and deletion, even by superuser
- `sudo chattr -i path/to/file_or_directory` : make a file or directory mutable
- `sudo chattr -R +i path/to/directory` : recursively make an entire directory and contents immutable
- `chattr +F path/to/directory` : mark a directory and its files to be interpreted in a case-insensitive manner (case-Folding)
- `sudo chattr +a path/to/file` : set a file to only allow appending

## See also
- lsattr

## Categories
files, users

## Links
- More information: https://manned.org/chattr
- tldr page: https://tldr.inbrowser.app/pages/linux/chattr
