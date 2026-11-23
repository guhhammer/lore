# docker container stats

Affiche un flux en direct des statistiques d'utilisation des ressources pour les conteneurs.

## Usage
- `docker stats` : affiche un flux en direct des statistiques d'utilisation des ressources pour tous les conteneurs
- `docker stats nom_du_conteneur` : affiche un flux en direct des statistiques d'utilisation des ressources pour un ou plusieurs conteneurs séparés par des espaces
- `docker stats --format "\{\{.Name\}\}:\t\{\{.CPUPerc\}\}"` : change le format de sortie pour afficher l'utilisation CPU du conteneur en pourcentage
- `docker stats -a` : affiche les statistiques d'utilisation des ressources pour tous les conteneurs (y compris ceux qui ne sont pas en cours d'exécution)
- `docker stats --no-stream` : désactive le flux en direct des statistiques d'utilisation des ressources et affiche les statistiques une seule fois

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/stats/
