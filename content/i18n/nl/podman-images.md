# podman images

Beheer OCI/Docker container-images.

## Usage
- `podman images` : toon alle container-images
- `podman images -a` : toon alle container-images, inclusief tussenliggende
- `podman images -q` : toon de uitvoer in stille modus (alleen numerieke ID's)
- `podman images -f dangling=true` : toon alle images die door geen enkele container worden gebruikt
- `podman images "*image|image*"` : toon images die een substring in hun naam bevatten

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-images.1.html
