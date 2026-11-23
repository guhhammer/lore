# gksudo

Frontend for `sudo`. Run graphical commands which need root access without having to run an X terminal emulator. Note: This command is deprecated and no longer maintained, use `pkexec` instead.

## Usage
- `gksudo -u userid command` : run a command as a specific user
- `gksudo -u userid -k command` : run the command while preserving the current environments
- `gksu -u userid -w command` : force `gksudo` to use `su` to run the command
- `gksudo -u userid -S command` : force `gksudo` to use `sudo` to run the command
- `gksudo -u userid -d command` : output debug info for the given command

## See also
- gksu

## Categories
users, shell

## Links
- More information: https://manned.org/gksudo
- tldr page: https://tldr.inbrowser.app/pages/linux/gksudo
