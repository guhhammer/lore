# podman-compose

Exécute et gère des conteneurs définis selon la spécification Compose.

## Usage
- `podman-compose ps` : liste tous les conteneurs en cours d'exécution
- `podman-compose up -d` : crée et démarre tous les conteneurs en arrière-plan en utilisant un fichier `docker-compose.yml` local
- `podman-compose up --build` : démarre tous les conteneurs en les construisant si nécessaire
- `podman-compose -f chemin/vers/fichier.yaml up` : démarre tous les conteneurs en utilisant un fichier compose alternatif
- `podman-compose stop` : arrête tous les conteneurs en cours d'exécution
- `podman-compose down -v` : supprime tous les conteneurs, réseaux et volumes
- `podman-compose logs -f nom_conteneur` : suit les journaux d'un conteneur (omettre tous les noms de conteneurs)
- `podman-compose run nom_service commande` : exécute une commande ponctuelle dans un service sans ports associés

## Links
- More information: https://github.com/containers/podman-compose
