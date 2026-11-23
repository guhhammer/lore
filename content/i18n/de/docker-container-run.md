# docker container run

Führe einen Befehl in einem neuen Docker Container aus.

## Usage
- `docker run image:tag befehl` : führe einen Befehl in einem neuen Docker Container aus und verwende dabei einen bestimmten Tag eines Docker Images
- `docker run -d image befehl` : führe einen Befehl in einem neuen Container im Hintergrund aus und zeige die ID
- `docker run --rm -it image befehl` : führe einen Befehl in einem kurzlebigen Container im interaktiven Modus mit einem Pseudo-TTY aus
- `docker run -e 'variable=wert' -e variable image befehl` : führe einen Befehl in einem neuen Container mit übergebenen Umgebungsvariablen aus
- `docker run -v /pfad/zu/host_verzeichnis:/pfad/zu/container_verzeichnis image befehl` : führe einen Befehl in einem neuen Container mit eingebundenen Datenträgern aus
- `docker run -p host_port:container_port image befehl` : führe einen Befehl in einem neuen Container mit veröffentlichten Ports aus
- `docker run --entrypoint befehl image` : führe einen Befehl in einem neuen Container aus und überschreibe den Einstiegspunkt des Images
- `docker run --network netzwerk image` : führe einen Befehl in einem neuen Container aus und verbinde ihn mit einem Netzwerk

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/run/
