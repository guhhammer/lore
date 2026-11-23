# docker container stats

Display a live stream of resource usage statistics for containers.

## Usage
- `docker stats` : display a live stream for the statistics of all running containers
- `docker stats container1 container2 ...` : display a live stream of statistics for one or more containers
- `docker stats --format "\{\{.Name\}\}:\t\{\{.CPUPerc\}\}"` : change the columns format to display container's CPU usage percentage
- `docker stats -a` : display statistics for all containers (both running and stopped)
- `docker stats --no-stream` : disable streaming stats and only pull the current stats

## Categories
containers, monitoring, desktop

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/stats/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-container-stats
