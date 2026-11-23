# docker

Gestion des conteneurs et des images Docker. Certaines commandes comme `docker run` ont leur propre documentation.

## Usage
- `docker ps -a` : liste tous les conteneurs Docker (en cours d'exécution ou arrêtés)
- `docker run --name nom_conteneur image` : démarre un conteneur à partir d'une image, avec un nom personnalisé
- `docker container start|stop nom_conteneur` : démarre ou arrête un conteneur existant
- `docker pull image` : télécharge une image depuis un registre Docker
- `docker images` : affiche les images déjà téléchargées
- `docker exec -it nom_conteneur sh` : ouvre un shell dans un conteneur déjà en cours d'exécution
- `docker rm nom_conteneur` : supprime un conteneur arrêté
- `docker logs -f nom_conteneur` : récupère et suit les journaux de message d'un conteneur

## Links
- More information: https://docs.docker.com/reference/cli/docker/
