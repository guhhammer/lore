# docker volume

Gére les volumes de Docker.

## Usage
- `docker volume create nom_du_volume` : crée un volume
- `docker volume create --label étiquette nom_du_volume` : crée un volume avec une étiquette spécifique
- `docker volume create -o type=tmpfs -o device=tmpfs -o o=size=100m,uid=1000 nom_du_volume` : crée un volume `tmpfs` avec une taille de 100 Mo et un uid de 1000
- `docker volume ls` : liste tous les volumes
- `docker volume rm nom_du_volume` : supprime un volume
- `docker volume inspect nom_du_volume` : affiche des informations sur un volume
- `docker volume prune` : supprime tous les volumes locaux non utilisés
- `docker volume sous_commande --help` : affiche l'aide pour une sous-commande

## Links
- More information: https://docs.docker.com/reference/cli/docker/volume/
