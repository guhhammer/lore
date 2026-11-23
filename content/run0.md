# run0

Elevate privileges interactively. Similar to `sudo`, but it's not a SUID binary, authentication takes place via polkit, and commands are invoked from a `systemd` service.

## Usage
- `run0 command` : run a command as root
- `run0 -u username|user_id -g group_name|group_id command` : run a command as another user and/or group

## See also
- sudo
- pkexec
- doas

## Categories
system, users

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/run0.html
- tldr page: https://tldr.inbrowser.app/pages/linux/run0
