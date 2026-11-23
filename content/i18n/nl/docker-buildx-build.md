# docker buildx build

Bouw een image vanuit een Dockerfile met de BuildKit-engine.

## Usage
- `docker buildx build .` : bouw een image vanuit de Dockerfile in de huidige map
- `docker buildx build -t image:tag .` : bouw een image en voorzie deze van een tag
- `docker buildx build -f pad/naar/Dockerfile .` : bouw een image met een specifieke Dockerfile
- `docker buildx build --build-arg HTTP_PROXY=http://proxy.example.com --build-arg VERSION=1.0 .` : bouw een image met build-time variabelen
- `docker buildx build --no-cache .` : bouw een image zonder de build-cache te gebruiken
- `docker buildx build --load -t image:tag .` : bouw een image en laad deze in `docker images`
- `docker buildx build --platform linux/amd64,linux/arm64 --push -t registry.example.com/image:tag .` : bouw voor meerdere platformen en push naar een registry
- `docker buildx build --target stage_naam -t image:tag .` : bouw een specifieke stage vanuit een multi-stage Dockerfile

## Links
- More information: https://docs.docker.com/reference/cli/docker/buildx/build
