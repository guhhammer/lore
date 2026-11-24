# podman rmi

Remove OCI/Docker images.

## Usage
- `podman rmi image1:tag image2:tag ...` : remove one or more images given their names
- `podman rmi -f image` : force remove an image
- `podman rmi --no-prune image` : remove an image without deleting untagged parents
- `podman rmi -h` : display help

## Categories
containers, media

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-rmi.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/podman-rmi
- Wikipedia: https://en.wikipedia.org/wiki/Podman
