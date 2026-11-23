# docker container exec

Exécute une commande dans un conteneur déjà en cours d'exécution.

## Usage
- `docker exec -it nom_conteneur /bin/bash` : entre dans un shell interactif dans un conteneur en cours d'exécution
- `docker exec -d nom_conteneur commande` : exécute une commande en arrière-plan (détachée) dans un conteneur en cours d'exécution
- `docker exec -it -w chemin/vers/répertoire nom_conteneur commande` : sélectionne le répertoire de travail pour une commande donnée à exécuter
- `docker exec -i -d nom_conteneur commande` : exécute une commande en arrière-plan sur un conteneur existant mais garde `stdin` ouvert
- `docker exec -it -e variable_environnement=valeur nom_conteneur /bin/bash` : définit une variable d'environnement dans une session Bash en cours d'exécution
- `docker exec -u utilisateur nom_conteneur commande` : exécute une commande en tant qu'utilisateur spécifique

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/exec/
