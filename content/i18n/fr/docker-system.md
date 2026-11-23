# docker system

Gère les données Docker et affiche des informations sur l'ensemble du système.

## Usage
- `docker system df` : affiche l'utilisation du disque par Docker
- `docker system df -v` : affiche des informations détaillées sur l'utilisation du disque par Docker
- `docker system prune` : supprime les données non utilisées (ajouter `--volumes` pour supprimer également des volumes non utilisés)
- `docker system prune --filter "until=heureshminutesm"` : supprime les données non utilisées de plus d'un temps donné dans le passé
- `docker system prune -a --volumes` : supprime tous les données non utilisées
- `docker system events` : affiche les événements en temps réel du démon Docker
- `docker system events -f 'type=container' --format '\{\{json .\}\}'` : affiche les événements en temps réel des conteneurs sous forme de JSON
- `docker system info` : affiche les informations générales du système

## Links
- More information: https://docs.docker.com/reference/cli/docker/system/
