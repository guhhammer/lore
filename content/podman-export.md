# podman export

Export the filesystem of a container and save it as a tarball on the local machine.

## Usage
- `podman export -o path/to/file.tar container_name_or_id` : export a container's filesystem to a `.tar` file
- `podman export container_name_or_id >path/to/file.tar` : export a container's filesystem to `stdout`

## See also
- podman import
- podman save

## Categories
containers, disks, files

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-export.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/podman-export
