# gksu

Frontend for `su`. Run graphical commands which need root access without having to run an X terminal emulator. Note: This command is deprecated and no longer maintained, use `pkexec` instead.

## Usage
- `gksu -u userid command` : run a command as a specific user
- `gksu -u userid -k command` : run the command while preserving the current environments
- `gksu -u userid -w command` : force `gksu` to use `su` to run the command
- `gksu -u userid -S command` : force `gksu` to use `sudo` to run the command
- `gksu -u userid -d command` : output debug info for the given command

## See also
- gksudo

## Categories
shell

## Links
- More information: https://manned.org/gksu
- tldr page: https://tldr.inbrowser.app/pages/linux/gksu
