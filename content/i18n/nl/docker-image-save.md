# docker image save

Exporteer Docker-images naar archief.

## Usage
- `docker save image:tag >pad/naar/bestand.tar` : sla een image op door `stdout` om te leiden naar een `.tar` archief
- `docker save -o pad/naar/bestand.tar image:tag` : sla een image op in een `.tar` archief
- `docker save -o pad/naar/bestand.tar image_naam` : sla alle tags van het image op
- `docker save -o pad/naar/bestand.tar image_naam:tag1 image_naam:tag2 ...` : kies bepaalde tags van een image om op te slaan

## Links
- More information: https://docs.docker.com/reference/cli/docker/image/save/
