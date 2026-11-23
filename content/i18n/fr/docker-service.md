# docker service

Gére les services sur un démon Docker.

## Usage
- `docker service ls` : liste les services sur un démon Docker
- `docker service create --name nom_du_service image:etiquette` : crée un nouveau service
- `docker service inspect nom_du_service|id` : affiche des informations détaillées d'une liste de services séparée par des espaces
- `docker service ps nom_du_service|id` : liste les tâches d'une liste de services séparée par des espaces
- `docker service scale nom_du_service=count_of_replicas` : redimensionne à un nombre spécifique de réplicas pour une liste de services séparée par des espaces
- `docker service rm nom_du_service|id` : supprime une liste de services séparée par des espaces

## Links
- More information: https://docs.docker.com/reference/cli/docker/service/
