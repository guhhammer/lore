# docker container

Gère les conteneurs Docker.

## Usage
- `docker ps` : liste les conteneurs Dockers en cours d'exécution
- `docker start nom_conteneur_1 nom_conteneur_2 ...` : démarre un ou plusieurs conteneur arrêtés
- `docker kill nom_conteneur_1 nom_conteneur_2 ...` : tue un ou plusieurs conteneurs en cours d'exécution
- `docker stop nom_conteneur_1 nom_conteneur_2 ...` : arrête un ou plusieurs conteneurs en cours d'exécution
- `docker pause nom_conteneur_1 nom_conteneur_2 ...` : mets en pause tous les processus d'un ou plusieurs conteneurs
- `docker container inspect nom_conteneur_1 nom_conteneur_2 ...` : affiche des informations détaillées sur un ou plusieurs conteneurs
- `docker export nom_conteneur` : exporte le système de fichiers d'un conteneur sous forme d'archive `.tar`
- `docker commit nom_conteneur` : crée une nouvelle image à partir des changements d'un conteneur

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/
