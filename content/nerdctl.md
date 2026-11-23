# nerdctl

Docker-compatible CLI for containerd.

## Usage
- `nerdctl ps -a` : list all containers (running and stopped)
- `nerdctl run --name container_name image` : start a container from an image, with a custom name
- `nerdctl start|stop container_name` : start or stop an existing container
- `nerdctl pull image` : pull an image from a container registry
- `nerdctl images` : display the list of already downloaded images
- `nerdctl exec -it container_name sh` : open an interactive tty with Bourne shell (`sh`) inside a running container
- `nerdctl rm container1 container2 ...` : remove stopped containers
- `nerdctl logs -f container_name` : fetch and follow the logs of a container

## Categories
containers, media

## Links
- More information: https://github.com/containerd/nerdctl/blob/main/docs/command-reference.md
- tldr page: https://tldr.inbrowser.app/pages/linux/nerdctl
