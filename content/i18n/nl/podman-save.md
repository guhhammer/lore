# podman save

Sla een image op in een lokaal bestand of map.

## Usage
- `podman save -o pad/naar/bestand.tar image:tag` : sla een image op in een `.tar`-bestand
- `podman save image:tag >pad/naar/bestand.tar` : sla een image op in `stdout`
- `podman save image:tag | gzip|bzip2|xz|zstd|zstdchunked >pad/naar/bestand.tar.ext` : sla een image op met compressie
- `podman save image:tag | zstd -T 0 --ultra | pv | ssh gebruikersnaam@externe_host podman load` : zet een image over naar een systeem op afstand met on-the-fly compressie en voortgangsbalk

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-save.1.html
