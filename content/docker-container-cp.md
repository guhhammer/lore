# docker container cp

Copy files or directories between host and container filesystems.

## Usage
- `docker cp path/to/file_or_directory_on_host container_name:path/to/file_or_directory_in_container` : copy a file or directory from the host to a container
- `docker cp container_name:path/to/file_or_directory_in_container path/to/file_or_directory_on_host` : copy a file or directory from a container to the host
- `docker cp -L path/to/symlink_on_host container_name:path/to/file_or_directory_in_container` : copy a file or directory from the host to a container, following symlinks (copies the symlinked files directly, not the symlinks themselves)

## Categories
files, containers, network

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/cp/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-container-cp
