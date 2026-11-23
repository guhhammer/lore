# docker container ls

Liste Docker Container.

## Usage
- `docker ps` : liste zur Zeit laufende Container auf
- `docker ps -a` : liste laufende und gestoppte Container auf
- `docker ps -l` : zeige den zuletzt erstellten Container (berücksichtigt jeden Status)
- `docker ps -f "name=name"` : zeige nur Container mit einer bestimmten Zeichenkette im Namen
- `docker ps -f "ancestor=image:tag"` : zeige nur Container die von einem bestimmten Image abstammen
- `docker ps -f "exited=code" -a` : zeige nur Container mit einem bestimmten Exit-Code
- `docker ps -f "status=status"` : zeige nur Container mit einem bestimmten Status (created, running, removing, paused, exited und dead)
- `docker ps -f "volume=pfad/zu/verzeichnis" --format "table \{\{.ID\}\}\t\{\{.Image\}\}\t\{\{.Names\}\}\t\{\{.Mounts\}\}"` : zeige nur Container, welche einen bestimmten Datenträger oder einen Datenträger an einem bestimmten Pfad eingehängt haben

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/ls/
