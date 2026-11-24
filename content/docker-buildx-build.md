# docker buildx build

Build an image from a Dockerfile using the BuildKit engine.

## Usage
- `docker buildx build .` : build an image from the Dockerfile in the current directory
- `docker buildx build -t image:tag .` : build an image and tag it
- `docker buildx build -f path/to/Dockerfile .` : build an image using a specific Dockerfile
- `docker buildx build --build-arg HTTP_PROXY=http://proxy.example.com --build-arg VERSION=1.0 .` : build an image passing build-time variables
- `docker buildx build --no-cache .` : build an image without using the build cache
- `docker buildx build --load -t image:tag .` : build an image and load it into `docker images`
- `docker buildx build --platform linux/amd64,linux/arm64 --push -t registry.example.com/image:tag .` : build for multiple platforms and push to a registry
- `docker buildx build --target stage_name -t image:tag .` : build a specific stage from a multi-stage Dockerfile

## Categories
containers, development, media

## Links
- More information: https://docs.docker.com/reference/cli/docker/buildx/build
- tldr page: https://tldr.inbrowser.app/pages/common/docker-buildx-build
- Wikipedia: https://en.wikipedia.org/wiki/Docker_(software)
