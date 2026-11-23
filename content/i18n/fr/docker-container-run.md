# docker container run

Exécute une commande dans un nouveau conteneur Docker.

## Usage
- `docker run image:étiquette commande` : exécute une commande dans un nouveau conteneur Docker avec une image étiquetée
- `docker run -d image commande` : exécute une commande dans un nouveau conteneur Docker en mode détaché (en arrière-plan) et affiche l'ID du conteneur
- `docker run --rm -it image commande` : exécute une command dans un conteneur éphémère avec une mode interactif et un terminal pseudo-TTY
- `docker run -e 'variable=valeur' -e variable image commande` : exécute une commande dans un nouveau conteneur avec des variables d'environnement
- `docker run -v /chemin/vers/hôte:/chemin/vers/conteneur image commande` : exécute une commande dans un nouveau conteneur avec des volumes montés
- `docker run -p port_hôte:port_conteneur image commande` : exécute une commande dans un nouveau conteneur avec des ports publiés
- `docker run --entrypoint commande image` : exécute une commande dans un nouveau conteneur en écrasant l'entrée du point d'entrée de l'image
- `docker run --network reseau image` : exécute une commande dans un nouveau conteneur en le connectant à un réseau

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/run/
