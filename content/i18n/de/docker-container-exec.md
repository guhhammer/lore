# docker container exec

Führe Befehle in einem bereits laufenden Docker Container aus.

## Usage
- `docker exec -it container_name /bin/bash` : öffne eine Shell innerhalb eines bereits laufenden Containers
- `docker exec -d container_name befehl` : führe einen Befehl im Hintergrund (losgelöst) in einem laufenden Container aus
- `docker exec -it -w pfad/zu/verzeichnis container_name befehl` : bestimme das Arbeitsverzeichnis, in dem der Befehl ausgeführt werden soll
- `docker exec -i -d container_name befehl` : führe einen Befehl im Hintergrund in einem laufenden Container aus, aber lies von der Standardeingabe
- `docker exec -it -e variablen_name=wert container_name /bin/bash` : setze eine Umgebungsvariable in einer laufenden Bash Sitzung
- `docker exec -u benutzer container_name befehl` : führe einen Befehl als ein bestimmter Benutzer aus

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/exec/
