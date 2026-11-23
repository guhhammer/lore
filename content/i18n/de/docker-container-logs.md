# docker container logs

Zeige Container Logs.

## Usage
- `docker logs container_name` : zeige die Logs eines Containers
- `docker logs container_name -f` : zeige die Logs und aktualisiere sie automatisch
- `docker logs container_name -n 5` : zeige die letzten 5 Zeilen
- `docker logs container_name -t` : zeige die Logs und füge ihnen Zeitstempel hinzu
- `docker logs container_name --until time` : zeige Logs vor einem bestimmten Zeitpunkt der Ausführung des Containers (bspw. 23m, 10s, 2013-01-02T13:23:37)

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/logs/
