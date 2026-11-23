# homectl

Create, remove, change, or inspect home directories using the systemd-homed service.

## Usage
- `homectl list` : list user accounts and their associated home directories
- `sudo homectl create username` : create a user account and their associated home directory
- `sudo homectl remove username` : remove a specific user and the associated home directory
- `sudo homectl passwd username` : change the password for a specific user
- `sudo homectl with username -- command command_arguments` : run a shell or a command with access to a specific home directory
- `sudo homectl lock|unlock username` : lock or unlock a specific home directory
- `sudo homectl resize username 100G` : change the disk space assigned to a specific home directory to 100 GiB
- `homectl -h` : display help

## Categories
users, files, system

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/homectl.html
- tldr page: https://tldr.inbrowser.app/pages/linux/homectl
