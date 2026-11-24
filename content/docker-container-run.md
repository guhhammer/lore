# docker container run

Run a command in a new Docker container.

## Usage
- `docker run image:tag command` : run command in a new container from a tagged image
- `docker run -d image command` : run command in a new container in background and display its ID
- `docker run --rm -it image command` : run command in a one-off container in interactive mode and pseudo-TTY
- `docker run -e 'variable=value' -e variable image command` : run command in a new container with passed environment variables
- `docker run -v /path/to/host_directory:/path/to/container_directory image command` : run command in a new container with bind mounted volumes
- `docker run -p host_port:container_port image command` : run command in a new container with published ports
- `docker run --entrypoint command image` : run command in a new container overwriting the entrypoint of the image
- `docker run --network network image` : run command in a new container connecting it to a network

## Categories
containers, media, network

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/run/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-container-run
- Wikipedia: https://en.wikipedia.org/wiki/Docker_(software)
