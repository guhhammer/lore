# podman

Simple management tool for pods, containers, and images. Provides a Docker-CLI comparable command-line. Simply put: `alias docker=podman`.

## Usage
- `podman ps -a` : list all containers (both running and stopped)
- `podman run --name container_name image` : create a container from an image, with a custom name
- `podman start|stop container_name` : start or stop an existing container
- `podman pull image` : pull an image from a registry (defaults to Docker Hub)
- `podman images` : display the list of already downloaded images
- `podman exec -it container_name sh` : open a shell inside an already running container
- `podman rm container_name` : remove a stopped container
- `podman logs -f container_name_or_id1 container_name_or_id2 ...` : display the logs of one or more containers and follow log output

## Categories
containers

## Links
- More information: https://docs.podman.io/en/latest/Commands.html
- tldr page: https://tldr.inbrowser.app/pages/common/podman
- Wikipedia: https://en.wikipedia.org/wiki/Podman
