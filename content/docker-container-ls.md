# docker container ls

List Docker containers.

## Usage
- `docker ps` : list currently running Docker containers
- `docker ps -a` : list all Docker containers (running and stopped)
- `docker ps -l` : show the latest created container (includes all states)
- `docker ps -f "name=name"` : filter containers that contain a substring in their name
- `docker ps -f "ancestor=image:tag"` : filter containers that share a given image as an ancestor
- `docker ps -f "exited=code" -a` : filter containers by exit status code
- `docker ps -f "status=status"` : filter containers by status (created, running, removing, paused, exited, and dead)
- `docker ps -f "volume=path/to/directory" --format "table \{\{.ID\}\}\t\{\{.Image\}\}\t\{\{.Names\}\}\t\{\{.Mounts\}\}"` : filter containers that mount a specific volume or have a volume mounted in a specific path

## Categories
containers, disks, files

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/ls/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-container-ls
- Wikipedia: https://en.wikipedia.org/wiki/Docker_(software)
