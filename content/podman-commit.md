# podman commit

Create a new image based on the changed container.

## Usage
- `podman commit container image:tag` : create an image from a specific container
- `podman commit -c "ENV name=value" container image:tag` : apply an `ENV` instruction to the created image
- `podman commit -c CMD=command -c ENTRYPOINT=command -c "LABEL key=value" container image:tag` : apply `LABEL`, `ENTRYPOINT`, and `CMD` instruction to the created image
- `podman commit -a "author" -m "comment" container image:tag` : create an image with a specific author and comment in the metadata
- `podman commit -p false container image:tag` : create an image without pausing the container during commit
- `podman commit --help` : display help

## Categories
containers, media, version-control

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-commit.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/podman-commit
- Wikipedia: https://en.wikipedia.org/wiki/Podman
