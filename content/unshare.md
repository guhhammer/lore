# unshare

Execute a command in new user-defined namespaces.

## Usage
- `unshare` : execute the default shell
- `sudo unshare -n command argument1 argument2 ...` : execute a command without sharing access to connected networks
- `sudo unshare -mpnf command argument1 argument2 ...` : execute a command as a child process without sharing mounts, processes, or networks
- `sudo unshare -puinmf chroot path/to/new_root /bin/sh` : create a container manually

## Categories
users

## Links
- More information: https://manned.org/unshare
- tldr page: https://tldr.inbrowser.app/pages/linux/unshare
