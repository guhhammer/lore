# podman import

Import a tarball and save it as a filesystem image.

## Usage
- `podman import path/to/tarball.tar image:tag` : import a tarball from a local file and create an image
- `podman import https://example.com/image.tar image:tag` : import a tarball from a URL
- `podman import -m "commit_message" path/to/tarball.tar image:tag` : import a tarball and add a commit message
- `podman import -c CMD=/bin/bash path/to/tarball.tar image:tag` : import a tarball and set a default command (required for running the container)

## See also
- podman export
- podman save

## Categories
containers, media, archives

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-import.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/podman-import
