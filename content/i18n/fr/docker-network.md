# docker network

Crée et gére des réseaux Docker.

## Usage
- `docker network ls` : liste tous les réseaux disponible et configuré du service Docker
- `docker network create -d nom_du_driver nom_du_reseau` : crée un réseau défini par l'utilisateur
- `docker network inspect nom_du_reseau` : affiche les informations détaillées des réseaux séparés par des espaces
- `docker network connect nom_du_reseau nom_du_conteneur|id` : connecte un conteneur à un réseau en utilisant un nom ou un ID
- `docker network disconnect nom_du_reseau nom_du_conteneur|id` : déconnecte un conteneur d'un réseau en utilisant un nom ou un ID
- `docker network prune` : supprime tous les réseaux non utilisés (non reliés à un conteneur)
- `docker network rm nom_du_reseau` : supprime les réseaux séparés par des espaces

## Links
- More information: https://docs.docker.com/reference/cli/docker/network/
