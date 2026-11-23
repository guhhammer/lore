# lxc-start

Start a container.

## Usage
- `systemctl start lxc-net` : start the lxc service
- `sudo lxc-start container_name` : start a container
- `sudo lxc-start container_name -F` : start a container in the foreground
- `sudo lxc-stop container_name` : exit out of a foreground container (run this in a separate terminal)
- `sudo lxc-start container_name -l DEBUG -o path/to/logfile` : write debug logs to a file
- `lxc-start -?` : display help

## Categories
containers, users

## Links
- More information: https://linuxcontainers.org/lxc/getting-started/
- tldr page: https://tldr.inbrowser.app/pages/linux/lxc-start
