# podman run

Run a command in a new Podman container.

## Usage
- `podman run image:tag command` : run command in a new container from a tagged image
- `podman run -d image:tag command` : run command in a new container in background and display its ID
- `podman run --rm -it image:tag command` : run command in a one-off container in interactive mode and pseudo-TTY
- `podman run -e 'variable=value' -e variable image:tag command` : run command in a new container with passed environment variables
- `podman run -v /path/to/host_directory:/path/to/container_directory image:tag command` : run command in a new container with bind mounted volumes
- `podman run -p host_port:container_port image:tag command` : run command in a new container with published ports
- `podman run --entrypoint command image:tag` : run command in a new container overwriting the entrypoint of the image
- `podman run --network network image:tag` : run command in a new container connecting it to a network

## Categories
containers, media, network

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-run.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/podman-run
- Wikipedia: https://en.wikipedia.org/wiki/Podman
