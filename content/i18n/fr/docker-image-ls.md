# docker image ls

Gérer les images Docker.

## Usage
- `docker images` : lister toutes les images Docker
- `docker images -a` : lister toutes les images Docker, y compris les intermédiaires
- `docker images -q` : lister les images Docker en mode silencieux (seulement les IDs numériques)
- `docker images -f dangling=true` : lister toutes les images Docker non utilisées par un conteneur
- `docker images "*nom*"` : lister les images Docker qui contiennent une sous-chaîne dans leur nom

## Links
- More information: https://docs.docker.com/reference/cli/docker/image/ls/
