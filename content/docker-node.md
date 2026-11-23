# docker node

Manage Docker Swarm nodes.

## Usage
- `docker node ls` : list nodes in the swarm
- `docker node ps node1 node2 node3 ...` : list tasks running on one or more nodes, defaults to the current node
- `docker node inspect node1 node2 node3 ...` : display detailed information on one or more nodes
- `docker node promote node1 node2 node3 ...` : promote one or more nodes to manager in the swarm
- `docker node demote node1 node2 node3 ...` : demote one or more nodes from manager in the swarm
- `docker node rm node1 node2 node3 ...` : remove one or more nodes from the swarm
- `docker node update --availability|role|label-add|... active|worker|... node1` : update metadata about a node, such as its availability, labels, or roles

## Categories
containers

## Links
- More information: https://docs.docker.com/reference/cli/docker/node/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-node
