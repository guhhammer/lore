# systemctl exit

Ask the service manager to quit.

## Usage
- `systemctl exit --user` : exit the user service manager
- `systemctl exit code --user` : exit the user service manager with a specific exit code
- `systemctl exit` : ask the container's service manager to exit (equivalent of `systemctl poweroff` if not in a container)

## Categories
system, users

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-exit
