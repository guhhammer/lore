# docker container cp

Kopieer bestanden of mappen tussen host- en containerbestandssystemen.

## Usage
- `docker cp pad/naar/bestand_of_map_op_host container_naam:pad/naar/bestand_of_map_in_container` : kopieer een bestand of map van de host naar een container
- `docker cp container_naam:pad/naar/bestand_of_map_in_container pad/naar/bestand_of_map_op_host` : kopieer een bestand of map van een container naar de host
- `docker cp -L pad/naar/symlink_op_host container_naam:pad/naar/bestand_of_map_in_container` : kopieer een bestand of map van de host naar een container, symlinks volgend (kopieert de symlinked bestanden direct, niet de symlinks zelf)

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/cp/
