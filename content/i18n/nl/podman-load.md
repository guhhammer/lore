# podman load

Laad een image uit een oci-archief of een docker-archief gemaakt met `podman save`.

## Usage
- `podman load -i pad/naar/bestand.tar` : laad een image vanuit een `.tar`-bestand
- `podman load -i pad/naar/bestand.tar.ext` : laad een image vanuit een gecomprimeerd `.tar`-bestand
- `podman load -q -i pad/naar/bestand.tar` : laad een image en toon stille uitvoer (toon alleen de image-ID)
- `podman <pad/naar/bestand.tar load` : laad een image van `stdin`

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-load.1.html
