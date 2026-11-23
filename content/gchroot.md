# gchroot

This command is an alias of GNU `chroot`.

## Usage
- `sudo chroot path/to/new_root` : run `$SHELL` in the new root directory
- `sudo chroot path/to/new_root command` : run command as new root directory
- `sudo chroot --userspec username_or_id:group_name_or_id path/to/new_root` : use a specific user and group

## Categories
files, users, shell, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/gchroot
