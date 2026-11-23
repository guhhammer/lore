# deluser

Delete a user from the system.

## Usage
- `sudo deluser username` : remove a user
- `sudo deluser --remove-home username` : remove a user and their home directory
- `sudo deluser --backup-to path/to/backup_directory --remove-home username` : remove a user and their home, but backup their files into a `.tar.gz` file in the specified directory
- `sudo deluser --remove-all-files username` : remove a user, and all files owned by them
- `sudo deluser username group` : remove a user from a group, without deleting the user

## Categories
danger, users, files

## Links
- More information: https://manned.org/deluser
- tldr page: https://tldr.inbrowser.app/pages/linux/deluser
