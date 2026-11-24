# docker system

Manage Docker data and display system-wide information.

## Usage
- `docker system df` : show Docker disk usage
- `docker system df -v` : show detailed information on disk usage
- `docker system prune` : remove unused data (append `--volumes` to remove unused volumes as well)
- `docker system prune --filter "until=hourshminutesm"` : remove unused data created more than a specified amount of time in the past
- `docker system prune -a --volumes` : remove all unused data
- `docker system events` : display real-time events from the Docker daemon
- `docker system events -f 'type=container' --format '\{\{json .\}\}'` : display real-time events from containers streamed as valid JSON Lines
- `docker system info` : display system-wide information

## Categories
containers, desktop, text

## Links
- More information: https://docs.docker.com/reference/cli/docker/system/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-system
- Wikipedia: https://en.wikipedia.org/wiki/Docker_(software)
