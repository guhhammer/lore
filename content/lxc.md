# lxc

Manage Linux containers using the lxd REST API. Any container names or patterns can be prefixed with the name of a remote server.

## Usage
- `lxc list match_string` : list local containers matching a string. Omit the string to list all local containers
- `lxc image list [remote:]match_string` : list images matching a string. Omit the string to list all images
- `lxc init [remote:]image container` : create a new container from an image
- `lxc start [remote:]container` : start a container
- `lxc stop [remote:]container` : stop a container
- `lxc info [remote:]container` : show detailed info about a container
- `lxc snapshot [remote:]container snapshot` : take a snapshot of a container
- `lxc exec [remote:]container command` : execute a specific command inside a container

## Categories
containers, media, text

## Links
- More information: https://manned.org/lxc
- tldr page: https://tldr.inbrowser.app/pages/linux/lxc
