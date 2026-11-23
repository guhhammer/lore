# docker network

Create and manage Docker networks.

## Usage
- `docker network ls` : list all available and configured networks on Docker daemon
- `docker network create -d driver_name network_name` : create a user-defined network
- `docker network inspect network_name1 network_name2 ...` : display detailed information about one or more networks
- `docker network connect network_name container_name|id` : connect a container to a network using a name or ID
- `docker network disconnect network_name container_name|id` : disconnect a container from a network
- `docker network prune` : remove all unused (not referenced by any container) networks
- `docker network rm network_name1 network_name2 ...` : remove one or more unused networks

## Categories
containers, network

## Links
- More information: https://docs.docker.com/reference/cli/docker/network/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-network
