# restorecond

Daemon that monitors file creation and automatically restores SELinux contexts. Useful for directories where files are frequently created with incorrect contexts.

## Usage
- `sudo restorecond` : start the `restorecond` daemon
- `sudo restorecond -v` : run `restorecond` in verbose mode to see restoration events
- `sudo restorecond -d` : run `restorecond` in debug mode
- `sudo restorecond -f restorecond_file` : use alternative `restorecond.conf` file
- `sudo systemctl status restorecond` : check the status of the restorecond service
- `sudo systemctl enable restorecond --now` : enable restorecond to start at boot

## See also
- restorecon
- semanage fcontext

## Categories
files, system, users

## Links
- More information: https://manned.org/restorecond
- tldr page: https://tldr.inbrowser.app/pages/linux/restorecond
