# docker compose start

Start existing containers for services.

## Usage
- `docker compose start` : start existing containers for all services
- `docker compose start service1 service2 ...` : start existing containers for one or more services
- `docker compose start --dry-run` : simulate starting existing containers
- `docker compose start --wait` : start existing containers and wait for services to be running or healthy
- `docker compose start --wait --wait-timeout seconds` : start existing containers and wait up to a specified number of seconds

## Categories
containers, system

## Links
- More information: https://docs.docker.com/reference/cli/docker/compose/start/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-compose-start
