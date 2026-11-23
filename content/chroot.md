# chroot

Run command or interactive shell with special root directory.

## Usage
- `sudo chroot path/to/new_root` : run `$SHELL` in the new root directory
- `sudo chroot path/to/new_root command` : run command as new root directory
- `sudo chroot --userspec username_or_id:group_name_or_id path/to/new_root` : use a specific user and group

## Categories
files, users, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/chroot
