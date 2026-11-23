# mkhomedir_helper

Create the user's home directory after creating the user.

## Usage
- `sudo mkhomedir_helper username` : create a home directory for a user based on `/etc/skel` with umask 022
- `sudo mkhomedir_helper username 037` : create a home directory for a user based on `/etc/skel` with all permissions for owner (0) and read permission for group (3)
- `sudo mkhomedir_helper username umask path/to/skeleton_directory` : create a home directory for a user based on a custom skeleton

## Categories
users, files

## Links
- More information: https://manned.org/mkhomedir_helper
- tldr page: https://tldr.inbrowser.app/pages/linux/mkhomedir_helper
