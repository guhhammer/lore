# docker container ls

Toon Docker-containers.

## Usage
- `docker ps` : toon momenteel draaiende Docker-containers
- `docker ps -a` : toon alle Docker-containers (actief en gestopt)
- `docker ps -l` : toon de laatst aangemaakte container (inclusief alle toestanden)
- `docker ps -f "name=naam"` : filter containers die een substring in hun naam bevatten
- `docker ps -f "ancestor=image:tag"` : filter containers die een bepaalde image als voorouder hebben
- `docker ps -f "exited=code" -a` : filter containers op exit-statuscode
- `docker ps -f "status=status"` : filter containers op status (created, running, removing, paused, exited en dead)
- `docker ps -f "volume=pad/naar/map" --format "table \{\{.ID\}\}\t\{\{.Image\}\}\t\{\{.Names\}\}\t\{\{.Mounts\}\}"` : filter containers die gekoppeld zijn aan een specifiek volume of waarvan het volume op een specifiek pad is gekoppeld

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/ls/
