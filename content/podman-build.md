# podman build

Daemonless tool for building container images.

## Usage
- `podman build path/to/directory` : create an image using a `Dockerfile` or `Containerfile` in the specified directory
- `podman build -t image_name:version path/to/directory` : create an image with a specified tag
- `podman build -f Containerfile.different .` : create an image from a non-standard file
- `podman build --no-cache path/to/directory` : create an image without using any previously cached images
- `podman build -q path/to/directory` : create an image suppressing all output

## Categories
containers, development, files

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-build.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/podman-build
- Wikipedia: https://en.wikipedia.org/wiki/Podman
