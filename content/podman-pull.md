# podman pull

Pull images from a container registry.

## Usage
- `podman pull image:tag` : pull a specific container image
- `podman pull -q image:tag` : pull a container image in quiet mode
- `podman pull -a image` : pull all tags of a container image
- `podman pull --platform linux/arm64 image:tag` : pull a container image for a specific platform
- `podman pull --tls-verify=false image:tag` : pull a container image without TLS verification
- `podman pull -h` : display help

## Categories
containers, media

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-pull.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/podman-pull
