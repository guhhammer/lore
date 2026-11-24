# docker compose down

Stop and remove containers, networks, images, and volumes created by `docker compose up`.

## Usage
- `docker compose down` : stop and remove all containers and networks
- `docker compose down --rmi all` : stop and remove containers, networks, and all images used by services
- `docker compose down --rmi local` : stop and remove containers, networks, and only images without a custom tag
- `docker compose down -v` : stop and remove containers, networks, and all volumes
- `docker compose down --remove-orphans` : stop and remove everything including orphaned containers
- `docker compose -f path/to/config down` : stop and remove containers using an alternate compose file
- `docker compose down -t timeout` : stop and remove containers with a custom timeout in seconds
- `docker compose down --remove-orphans -v` : remove containers for services not defined in the Compose file

## Categories
containers, media, files

## Links
- More information: https://docs.docker.com/reference/cli/docker/compose/down/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-compose-down
- Wikipedia: https://en.wikipedia.org/wiki/Docker_(software)
