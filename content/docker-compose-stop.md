# docker compose stop

Stop running containers without removing them.

## Usage
- `docker compose stop` : stop all running services
- `docker compose stop service_1 service_2 ...` : stop specific services
- `docker compose stop -t seconds` : stop with a custom shutdown timeout in seconds
- `docker compose -f path/to/compose_file stop` : stop services defined in a specific compose file
- `docker compose stop --dry-run` : simulate running the operations without executing
- `docker compose stop -t seconds service_1 service_2 ...` : stop specific services with a custom timeout

## Categories
containers, system

## Links
- More information: https://docs.docker.com/reference/cli/docker/compose/stop
- tldr page: https://tldr.inbrowser.app/pages/common/docker-compose-stop
- Wikipedia: https://en.wikipedia.org/wiki/Docker_(software)
