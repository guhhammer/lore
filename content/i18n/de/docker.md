# docker

Verwalte Docker Container und Images. Manche Unterbefehle wie `run` sind separat dokumentiert.

## Usage
- `docker ps -a` : liste laufende und gestoppte Container auf
- `docker run --name container_name image` : erzeuge einen Container aus einem Image und benenne ihn
- `docker container start|stop container_name` : stoppe oder starte einen existierenden Container
- `docker pull image` : lade ein Image herunter
- `docker images` : zeige eine Liste der bereits heruntergeladenen Images an
- `docker exec -it container_name sh` : öffne eine Konsole innerhalb eines bereits laufenden Containers
- `docker rm container_name` : lösche einen gestoppten Container
- `docker logs -f container_name` : zeige die Logs eines Containers an und aktualisiere sie automatisch

## Links
- More information: https://docs.docker.com/reference/cli/docker/
