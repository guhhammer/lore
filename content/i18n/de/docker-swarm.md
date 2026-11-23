# docker swarm

Ein Container-Orchestrierungswerkzeug.

## Usage
- `docker swarm init` : initialisiere ein Swarm Cluster
- `docker swarm join-token worker|manager` : zeige den Token um als Manager oder Worker beizutreten
- `docker swarm join --token token manager_node_url:2377` : füge dem Cluster eine neue Node hinzu
- `docker swarm leave` : entferne einen Worker vom Swarm (führe dies auf der Worker Node aus)
- `docker swarm ca` : zeige die aktuellen CA Zertifikate im PEM Format
- `docker swarm ca --rotate` : rotiere die aktuellen CA Zertifikate und zeige die neuen Zertifikate
- `docker swarm update --cert-expiry stundenhminutenmsekundens` : ändere die Gültigkeitsdauer für Node-Zertifikate

## Links
- More information: https://docs.docker.com/engine/swarm/
