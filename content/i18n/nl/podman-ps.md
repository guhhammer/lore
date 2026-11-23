# podman ps

Toon Podman-containers.

## Usage
- `podman ps` : toon momenteel draaiende Podman-containers
- `podman ps -a` : toon alle Podman-containers (actief en gestopt)
- `podman ps -l` : toon de laatst aangemaakte container (inclusief alle toestanden)
- `podman ps -f "name=naam"` : filter containers die een substring in hun naam bevatten
- `podman ps -f "ancestor=image:tag"` : filter containers die een bepaalde image als voorouder hebben
- `podman ps -a -f "exited=code"` : filter containers op exitstatuscode
- `podman ps -f "status=created|running|removing|paused|exited|dead"` : filter containers op status
- `podman ps -f "volume=pad/naar/map" --format "table \{\{.ID\}\}\t\{\{.Image\}\}\t\{\{.Names\}\}\t\{\{.Mounts\}\}"` : filter containers die een specifiek volume koppelen of waarvan het volume op een specifiek pad is gekoppeld

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-ps.1.html
