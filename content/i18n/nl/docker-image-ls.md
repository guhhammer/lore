# docker image ls

Toon Docker-images.

## Usage
- `docker images` : toon alle Docker-images
- `docker images -a` : toon alle Docker-images inclusief tussenproducten
- `docker images -q` : toon de uitvoer in stille modus (alleen numerieke ID's)
- `docker images -f dangling=true` : toon alle Docker-images die door geen enkele container worden gebruikt
- `docker images "*naam*"` : toon images die een substring in hun naam bevatten
- `docker images --format "\{\{.ID\}\}\t\{\{.Size\}\}\t\{\{.Repository\}\}:\{\{.Tag\}\}" | sort -k 2 -h` : sorteer images op grootte

## Links
- More information: https://docs.docker.com/reference/cli/docker/image/ls/
