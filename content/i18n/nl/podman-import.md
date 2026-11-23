# podman import

Importeer een tarball en sla het op als een bestandssysteem-image.

## Usage
- `podman import pad/naar/tarball.tar image:tag` : importeer een tarball van een lokaal bestand en maak een image
- `podman import https://example.com/image.tar image:tag` : importeer een tarball vanaf een URL
- `podman import -m "commit_bericht" pad/naar/tarball.tar image:tag` : importeer een tarball en voeg een commitbericht toe
- `podman import -c CMD=/bin/bash pad/naar/tarball.tar image:tag` : importeer een tarball en stel een standaard commando in (nodig voor het uitvoeren van de container)

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-import.1.html
