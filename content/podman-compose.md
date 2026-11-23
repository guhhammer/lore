# podman-compose

Run and manage Compose Specification container definition.

## Usage
- `podman-compose ps` : list all running containers
- `podman-compose up -d` : create and start all containers in the background using a local `docker-compose.yml`
- `podman-compose up --build` : start all containers, building if needed
- `podman-compose -f path/to/file.yaml up` : start all containers using an alternate compose file
- `podman-compose stop` : stop all running containers
- `podman-compose down -v` : remove all containers, networks, and volumes
- `podman-compose logs -f container_name` : follow logs for a container (omit all container names)
- `podman-compose run service_name command` : run a one-time command in a service with no ports mapped

## Categories
containers, files

## Links
- More information: https://github.com/containers/podman-compose
- tldr page: https://tldr.inbrowser.app/pages/common/podman-compose
