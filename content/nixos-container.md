# nixos-container

Start NixOS containers using Linux containers.

## Usage
- `sudo nixos-container list` : list running containers
- `sudo nixos-container create container_name --config-file path/to/nix_config_file` : create a NixOS container with a specific configuration file
- `sudo nixos-container start|stop|terminate|destroy|status container_name` : start, stop, terminate, or destroy a specific container
- `sudo nixos-container run container_name -- command command_arguments` : run a command in a running container
- `sudo $EDITOR /var/lib/container/container_name/etc/nixos/configuration.nix && sudo nixos-container update container_name` : update a container configuration
- `sudo nixos-container root-login container_name` : enter an interactive shell session on an already-running container

## Categories
containers, users, files

## Links
- More information: https://nixos.org/manual/nixos/stable/#ch-containers
- tldr page: https://tldr.inbrowser.app/pages/linux/nixos-container
