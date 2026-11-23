# podlet

Generate Podman Quadlet files from a Podman command, compose file, or existing object.

## Usage
- `podlet podman_command` : generate the equivalent Quadlet for a podman command
- `podlet -n name -d description podman_command` : generate a Quadlet with a custom name and a description
- `podlet -i --wanted-by unit --after unit podman_command` : add startup dependency and ordering declarations to the generated Quadlet
- `podlet compose` : generate Quadlets from a compose file in the current directory
- `podlet -u compose path/to/compose.yaml` : generate Quadlets for a compose file into the unit directory of the current user

## See also
- podman

## Categories
containers, files

## Links
- More information: https://github.com/containers/podlet
- tldr page: https://tldr.inbrowser.app/pages/linux/podlet
