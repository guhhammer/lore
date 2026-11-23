# docker container logs

Affiche les journaux d'un conteneur.

## Usage
- `docker logs nom_conteneur` : affiche les journaux d'un conteneur
- `docker logs nom_conteneur -f` : affiche les journaux d'un conteneur en les suivant
- `docker logs nom_conteneur -n 5` : affiche les 5 dernières lignes des journaux d'un conteneur
- `docker logs nom_conteneur -t` : affiche les journaux d'un conteneur avec l'horodatage
- `docker logs nom_conteneur --until temps` : affiche les journaux d'un conteneur depuis un certain temps (i.e. 23m, 10s, 2013-01-02T13:23:37)

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/logs/
