# docker compose

Exécute et gère des applications au travers de plusieurs conteneurs Docker. Certaines sous-commandes, telles que `up`, `down`, `logs`, `stop` disposent de leur propre documentation d’utilisation.

## Usage
- `docker compose ps` : liste tous les conteneurs en cours d'exécution
- `docker compose up -d` : crée et démarre en arrière-plan tous les conteneurs décrits dans le fichier `docker-compose.yml` du répertoire actuel
- `docker compose up --build` : démarre tous les conteneurs après les avoir recréés si nécessaire
- `docker compose -p nom_projet -f chemin/vers/fichier up` : démarre tous les conteneurs en spécifiant un nom de projet et un fichier compose alternatif
- `docker compose stop` : arrête tous les conteneurs en cours d'exécution
- `docker compose down --rmi all -v` : arrête et supprime tous les conteneurs, réseaux, images et volumes
- `docker compose logs -f` : affiche et suit la journalisation de tous les conteneurs
- `docker compose logs -f nom_conteneur` : affiche et suit la journalisation pour un conteneurs spécifique

## Links
- More information: https://docs.docker.com/reference/cli/docker/compose/
