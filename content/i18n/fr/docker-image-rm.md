# docker image rm

Supprime une ou plusieurs images Docker.

## Usage
- `docker rmi image1 image2 ...` : supprime une ou plusieurs images en fonction de leurs noms
- `docker rmi -f image` : supprime une image en forçant la suppression
- `docker rmi --no-prune image` : supprime une image sans supprimer les parents non étiquetés
- `docker rmi --help` : affiche l'aide

## Links
- More information: https://docs.docker.com/reference/cli/docker/image/rm/
