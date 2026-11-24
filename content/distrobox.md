# distrobox

Use any Linux distribution inside your terminal in a container. Install & use packages inside it while tightly integrating with the host OS, sharing storage (`home` directory) and hardware. Note: `distrobox` isn't a command, but a set of commands.

## Usage
- `distrobox-create container_name -i ubuntu:latest` : create a Distrobox container using the Ubuntu image
- `distrobox-create -c container_name cloned_container_name` : clone a Distrobox container
- `distrobox-list` : list all Distrobox containers
- `distrobox-list -v` : list all Distrobox containers with verbose information
- `distrobox-enter container_name` : enter a Distrobox container
- `distrobox-enter container_name -- sh -l` : enter a Distrobox container and run a command at login
- `distrobox-host-exec "command"` : execute command on the host system from inside the Distrobox container
- `distrobox-host-exec ls` : execute the `ls` command on the host system from inside the container

## Categories
containers, network, packages

## Links
- More information: https://github.com/89luca89/distrobox
- tldr page: https://tldr.inbrowser.app/pages/linux/distrobox
- Wikipedia: https://en.wikipedia.org/wiki/Distrobox
