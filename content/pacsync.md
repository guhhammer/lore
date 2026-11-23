# pacsync

Update sync databases.

## Usage
- `sudo pacsync` : update all sync databases
- `sudo pacsync repository1 repository2 ...` : update specific sync databases
- `sudo pacsync --force` : update sync databases even if they are already up-to-date
- `sudo pacsync --config path/to/pacman.conf` : update sync databases using a specific configuration file
- `sudo pacsync --updated` : update sync databases and return true only if a database was actually updated

## Categories
users

## Links
- More information: https://github.com/andrewgregory/pacutils/blob/master/doc/pacsync.pod
- tldr page: https://tldr.inbrowser.app/pages/linux/pacsync
