# docker image ls

List Docker images.

## Usage
- `docker images` : list all Docker images
- `docker images -a` : list all Docker images including intermediates
- `docker images -q` : list the output in quiet mode (only numeric IDs)
- `docker images -f dangling=true` : list all Docker images not used by any container
- `docker images "*name*"` : list images that contain a substring in their name
- `docker images --format "\{\{.ID\}\}\t\{\{.Size\}\}\t\{\{.Repository\}\}:\{\{.Tag\}\}" | sort -k 2 -h` : sort images by size

## Categories
containers, media

## Links
- More information: https://docs.docker.com/reference/cli/docker/image/ls/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-image-ls
- Wikipedia: https://en.wikipedia.org/wiki/Docker_(software)
