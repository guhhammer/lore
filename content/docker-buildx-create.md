# docker buildx create

Create a new builder instance.

## Usage
- `docker buildx create` : create a new builder instance using the default Docker context
- `docker buildx create --name builder_name` : create a new builder instance with a specific name
- `docker buildx create --name builder_name --use` : create a new builder instance and immediately set it as the current active builder
- `docker buildx create --driver docker-container|kubernetes|remote|...` : create a new builder instance using a specific driver (defaults to `docker`)
- `docker buildx create --platform linux/amd64,linux/arm64,...` : create a new builder instance with specific supported platforms
- `docker buildx create --name builder_name --append context|endpoint` : append a new node to an existing builder
- `docker buildx create --buildkitd-flags "--debug --debugaddr 0.0.0.0:6666"` : create a new builder instance with specific BuildKit daemon flags
- `docker buildx create --name builder_name --bootstrap` : create a new builder instance and boot it immediately

## Categories
containers

## Links
- More information: https://docs.docker.com/reference/cli/docker/buildx/create/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-buildx-create
- Wikipedia: https://en.wikipedia.org/wiki/Docker_(software)
