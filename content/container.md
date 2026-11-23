# container

Create and run Linux containers as lightweight virtual machines on macOS.

## Usage
- `container system start` : start the container services (required before running containers)
- `container run --rm -it image sh` : run a container from an image interactively and remove it after it exits
- `container run -d --name container_name -p 8080:80 image` : run a container in the background with a name and a published port
- `container exec -it container_name sh` : execute a command inside a running container
- `container ls -a` : list all containers (running and stopped)
- `container logs -f container_name` : follow the logs of a container
- `container build -t image_name path/to/directory` : build an image from a Dockerfile in a directory
- `container stop|delete container_name` : stop or delete a container

## Categories
containers, media, files, macos

## Links
- More information: https://github.com/apple/container
- tldr page: https://tldr.inbrowser.app/pages/osx/container
