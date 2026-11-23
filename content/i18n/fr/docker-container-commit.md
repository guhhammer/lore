# docker container commit

Crée une nouvelle image depuis les changements d'un conteneur.

## Usage
- `docker commit conteneur image:etiquette` : crée une image à partir d'un conteneur spécifique
- `docker commit -c "CMD commande" conteneur image:etiquette` : applique une instruction `CMD` du Dockerfile à l'image créée
- `docker commit -c "ENV name=value" conteneur image:etiquette` : applique une instruction `ENV` du Dockerfile à l'image créée
- `docker commit -a "auteur" conteneur image:etiquette` : crée une image avec un auteur spécifique dans les métadonnées
- `docker commit -m "commentaire" conteneur image:etiquette` : crée une image avec un commentaire spécifique dans les métadonnées
- `docker commit -p false conteneur image:etiquette` : crée une image sans mettre en pause le conteneur pendant la création
- `docker commit --help` : affiche l'aide

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/commit/
