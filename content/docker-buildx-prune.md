# docker buildx prune

Remove the build cache.

## Usage
- `docker buildx prune` : remove build cache for currently active builder
- `docker buildx prune --filter "type=source.local"` : remove cache records based on a specific filter
- `docker buildx prune --max-used-space 128mb` : remove the least recently used cache records until the cache size is below a specific limit
- `docker buildx prune --reserved-space 2gb` : remove the least recently used cache records until a specific amount of free disk space is available

## Categories
containers, development

## Links
- More information: https://docs.docker.com/reference/cli/docker/buildx/prune/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-buildx-prune
- Wikipedia: https://en.wikipedia.org/wiki/Docker_(software)
