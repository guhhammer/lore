# podman images

Manage OCI/Docker container images.

## Usage
- `podman images` : list all container images
- `podman images -a` : list all container images including intermediates
- `podman images -q` : list the output in quiet mode (only numeric IDs)
- `podman images -f dangling=true` : list all images not used by any container
- `podman images "*image|image*"` : list images that contain a substring in their name

## Categories
containers, media

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-images.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/podman-images
- Wikipedia: https://en.wikipedia.org/wiki/Podman
