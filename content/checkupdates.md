# checkupdates

Check pending updates in Arch Linux.

## Usage
- `checkupdates` : synchronize the database and list pending updates
- `checkupdates -n` : list pending updates without syncing the database
- `checkupdates -c` : display the list of pending updates if it differs from the last time this option was used
- `checkupdates -d` : list pending updates and download the packages to the `pacman` cache (`/var/cache/pacman/pkg`)
- `CHECKUPDATES_DB=path/to/directory checkupdates` : list pending updates using a specific `pacman` database
- `checkupdates -h` : display help

## Categories
databases, network

## Links
- More information: https://manned.org/checkupdates
- tldr page: https://tldr.inbrowser.app/pages/linux/checkupdates
