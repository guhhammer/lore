# docker container commit

Create a new image from a container's changes.

## Usage
- `docker commit container image:tag` : create an image from a specific container
- `docker commit -c "CMD command" container image:tag` : apply a `CMD` Dockerfile instruction to the created image
- `docker commit -c "ENV name=value" container image:tag` : apply an `ENV` Dockerfile instruction to the created image
- `docker commit -a "author" container image:tag` : create an image with a specific author in the metadata
- `docker commit -m "comment" container image:tag` : create an image with a specific comment in the metadata
- `docker commit -p false container image:tag` : create an image without pausing the container during commit
- `docker commit --help` : display help

## Categories
containers, media, version-control

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/commit/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-container-commit
- Wikipedia: https://en.wikipedia.org/wiki/Docker_(software)
