# docker container

Manage Docker containers.

## Usage
- `docker ps` : list currently running Docker containers
- `docker start container1_name container2_name ...` : start one or more stopped containers
- `docker kill container1_name container2_name ...` : kill one or more running containers
- `docker stop container1_name container2_name ...` : stop one or more running containers
- `docker pause container1_name container2_name ...` : pause all processes within one or more containers
- `docker container inspect container1_name container2_name ...` : display detailed information on one or more containers
- `docker export container_name` : export a container's filesystem as a `.tar` archive
- `docker commit container_name` : create a new image from a container's changes

## Categories
containers

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-container
- Wikipedia: https://en.wikipedia.org/wiki/Docker_(software)
