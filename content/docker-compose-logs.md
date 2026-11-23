# docker compose logs

View output from containers in a Docker Compose application.

## Usage
- `docker compose logs` : view logs for all services
- `docker compose logs service_name` : view logs for a specific service
- `docker compose logs -f` : view logs and follow new output (like `tail --follow`)
- `docker compose logs -t` : view logs with timestamps
- `docker compose logs -n n` : view only the last `n` lines of logs for each container
- `docker compose logs --since timestamp` : view logs from a specific time onwards
- `docker compose logs --until timestamp` : view logs until a specific time
- `docker compose logs service1 service2 ...` : view logs for multiple specific services

## Categories
containers, system

## Links
- More information: https://docs.docker.com/reference/cli/docker/compose/logs/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-compose-logs
