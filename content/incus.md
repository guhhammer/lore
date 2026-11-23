# incus

Modern, secure, and powerful system container and virtual machine manager.

## Usage
- `incus list` : list all containers and virtual machines (both running and stopped)
- `incus create image container_name` : create a container from an image, with a custom name
- `incus start|stop container_name` : start or stop an existing container
- `incus shell container_name` : open a shell inside an already running container
- `incus delete container_name` : remove a stopped container
- `incus copy remote:image local:custom_image_name` : pull an image from an image repository (remote) to local
- `incus image list images:` : list all available images in the official `images:` remote
- `incus image list local:` : list all images already downloaded to the `local:` remote

## Categories
containers, media, security

## Links
- More information: https://linuxcontainers.org/incus/docs/main/
- tldr page: https://tldr.inbrowser.app/pages/common/incus
