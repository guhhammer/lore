# podman build

Daemonloze tool voor het bouwen van container-images.

## Usage
- `podman build pad/naar/map` : maak een image aan met behulp van een `Dockerfile` of `Containerfile` in de opgegeven map
- `podman build -t image_naam:versie pad/naar/map` : maak een image aan met een opgegeven tag
- `podman build -f Containerfile.anders .` : maak een image aan vanuit een niet-standaard bestand
- `podman build --no-cache pad/naar/map` : maak een image aan zonder eerder gecachte images te gebruiken
- `podman build -q pad/naar/map` : maak een image aan waarbij alle uitvoer wordt onderdrukt

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-build.1.html
