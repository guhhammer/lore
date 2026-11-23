# docker compose

Run and manage multi container Docker applications. Some subcommands such as `up`, `down`, `logs`, `stop`, etc. have their own usage documentation.

## Usage
- `docker compose ps` : list all running containers
- `docker compose up -d` : create and start all containers in the background using a `docker-compose.yml` file from the current directory
- `docker compose up --build` : start all containers, rebuild if necessary
- `docker compose -p project_name -f path/to/file up` : start all containers by specifying a project name and using an alternate compose file
- `docker compose stop` : stop all running containers
- `docker compose down --rmi all -v` : stop and remove all containers, networks, images, and volumes
- `docker compose logs -f` : follow logs for all containers
- `docker compose logs -f container_name` : follow logs for a specific container

## Categories
containers, files, documents

## Links
- More information: https://docs.docker.com/reference/cli/docker/compose/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-compose
