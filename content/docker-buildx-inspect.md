# docker buildx inspect

Inspect the current or a specified builder instance.

## Usage
- `docker buildx inspect` : show information about the current builder instance
- `docker buildx inspect builder_name` : inspect a specific builder instance by name
- `docker buildx inspect --bootstrap` : ensure the builder is running before inspecting
- `docker buildx inspect --timeout seconds` : override the default timeout for loading builder status (default: 20 seconds)

## Categories
containers

## Links
- More information: https://docs.docker.com/reference/cli/docker/buildx/inspect
- tldr page: https://tldr.inbrowser.app/pages/common/docker-buildx-inspect
- Wikipedia: https://en.wikipedia.org/wiki/Docker_(software)
