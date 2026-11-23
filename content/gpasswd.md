# gpasswd

Administer `/etc/group` and `/etc/gshadow`.

## Usage
- `sudo gpasswd -A user1,user2 group` : define group administrators
- `sudo gpasswd -M user1,user2 group` : set the list of group members
- `gpasswd group` : create a password for the named group
- `gpasswd -a user group` : add a user to the named group
- `gpasswd -d user group` : remove a user from the named group

## Categories
users

## Links
- More information: https://manned.org/gpasswd
- tldr page: https://tldr.inbrowser.app/pages/linux/gpasswd
