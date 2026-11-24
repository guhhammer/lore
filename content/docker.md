# docker

Manage Docker containers and images. Some subcommands such as `container`, `image`, etc. have their own usage documentation.

## Usage
- `docker ps -a` : list all Docker containers (running and stopped)
- `docker run --name container_name image` : start a container from an image, with a custom name
- `docker container start|stop container_name` : start or stop an existing container
- `docker pull image` : pull an image from a Docker registry
- `docker images` : display the list of already downloaded images
- `docker exec -it container_name sh` : open an interactive tty with Bourne shell (`sh`) inside a running container
- `docker rm container1 container2 ...` : remove stopped containers
- `docker logs -f container_name` : fetch and follow the logs of a container

## Categories
containers

## Links
- More information: https://docs.docker.com/reference/cli/docker/
- tldr page: https://tldr.inbrowser.app/pages/common/docker
- Wikipedia: https://en.wikipedia.org/wiki/Docker_(software)
