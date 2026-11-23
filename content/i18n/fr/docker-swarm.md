# docker swarm

Outil d'orchestration de conteneurs.

## Usage
- `docker swarm init` : initialise un cluster swarm
- `docker swarm join-token worker|manager` : affiche le jeton pour rejoindre un cluster swarm en tant que nœud manager ou worker
- `docker swarm join --token jeton url_du_manager:2377` : rejoint un nouveau nœud au cluster
- `docker swarm leave` : supprime un worker du cluster (à exécuter dans le nœud worker)
- `docker swarm ca` : affiche le certificat CA actuel au format PEM
- `docker swarm ca --rotate` : change le certificat CA actuel et affiche le nouveau certificat
- `docker swarm update --cert-expiry heureshminutesmsecondess` : change la période de validité des certificats des nœuds

## Links
- More information: https://docs.docker.com/engine/swarm/
