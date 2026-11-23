# gchown

This command is an alias of GNU `chown`.

## Usage
- `sudo chown user path/to/file_or_directory` : change the owner user of a file/directory
- `sudo chown user:group path/to/file_or_directory` : change the owner user and group of a file/directory
- `sudo chown user: path/to/file_or_directory` : change the owner user and group to both have the name `user`
- `chown :group path/to/file_or_directory` : change the group of a file to a group that the current user belongs to
- `sudo chown -R user path/to/directory` : recursively change the owner of a directory and its contents
- `sudo chown -h user path/to/symlink` : change the owner of a symbolic link
- `sudo chown --reference path/to/reference_file path/to/file_or_directory` : change the owner of a file/directory to match a reference file

## Categories
users, files, shell, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/gchown
