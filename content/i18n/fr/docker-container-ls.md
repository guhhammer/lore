# docker container ls

Liste les conteneurs Docker.

## Usage
- `docker ps` : liste les conteneurs Docker en cours d'exécution
- `docker ps -a` : liste tous les conteneurs Docker (en cours d'exécution et arrêtés)
- `docker ps -l` : affiche le dernier conteneur Docker créé (avec tous les états)
- `docker ps -f "name=nom"` : affiche les conteneurs avec une chaine de caractère dans leur nom
- `docker ps -f "ancestor=image:etiquette"` : affiche les conteneurs avec une même image comme parent
- `docker ps -f "exited=code" -a` : affiche les conteneurs avec un code de sorti spécifique
- `docker ps -f "status=statut"` : affiche les conteneurs avec un statut spécifique (créé, en cours d'exécution, en cours de suppresion, en pause, arrêté, mort)
- `docker ps -f "volume=chemin/vers/répertoire" --format "table \{\{.ID\}\}\t\{\{.Image\}\}\t\{\{.Names\}\}\t\{\{.Mounts\}\}"` : affiche les conteneurs avec un point de montage spécifique

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/ls/
