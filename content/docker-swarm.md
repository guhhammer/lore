# docker swarm

A container orchestration tool.

## Usage
- `docker swarm init` : initialize a swarm cluster
- `docker swarm join-token worker|manager` : display the token to join a manager or a worker
- `docker swarm join --token token manager_node_url:2377` : join a new node to the cluster
- `docker swarm leave` : remove a worker from the swarm (run inside the worker node)
- `docker swarm ca` : display the current CA certificate in PEM format
- `docker swarm ca --rotate` : rotate the current CA certificate and display the new certificate
- `docker swarm update --cert-expiry hourshminutesmsecondss` : change the valid period for node certificates

## Categories
containers, desktop, security

## Links
- More information: https://docs.docker.com/engine/swarm/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-swarm
