# podman load

Load an image from an oci-archive or a docker-archive created using `podman save`.

## Usage
- `podman load -i path/to/file.tar` : load an image from a `.tar` file
- `podman load -i path/to/file.tar.ext` : load an image from a compressed `.tar` file
- `podman load -q -i path/to/file.tar` : load an image and display quiet output (only show the image ID)
- `podman <path/to/file.tar load` : load an image from `stdin`

## See also
- podman save
- podman import

## Categories
containers, monitoring, archives

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-load.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/podman-load
