# docker service

Manage the services on a Docker daemon.

## Usage
- `docker service ls` : list the services on a Docker daemon
- `docker service create --name service_name image:tag` : create a new service
- `docker service inspect service_name_or_id1 service_name_or_id2 ...` : display detailed information about one or more services
- `docker service ps service_name_or_id1 service_name_or_id2 ...` : list the tasks of one or more services
- `docker service scale service_name=count_of_replicas` : scale to a specific number of replicas for a space-separated list of services
- `docker service rm service_name_or_id1 service_name_or_id2 ...` : remove one or more services

## Categories
system, containers

## Links
- More information: https://docs.docker.com/reference/cli/docker/service/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-service
- Wikipedia: https://en.wikipedia.org/wiki/Docker_(software)
