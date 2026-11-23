# podman pull

Haal images op uit een containerregister.

## Usage
- `podman pull image:tag` : haal een specifieke container-image op
- `podman pull -q image:tag` : haal een container-image in stille modus op
- `podman pull -a image` : haal alle tags van een container-image op
- `podman pull --platform linux/arm64 image:tag` : haal een container-image op voor een specifiek platform
- `podman pull --tls-verify=false image:tag` : haal een container-image op zonder TLS-verificatie
- `podman pull -h` : toon de help

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-pull.1.html
