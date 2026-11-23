# docker container

Verwalte Docker Container.

## Usage
- `docker ps` : liste zur Zeit laufende Container auf
- `docker start container1_name container2_name ...` : starte einen oder mehrere gestoppte Container
- `docker kill container1_name container2_name ...` : beende einen oder mehrere laufende Container sofort
- `docker stop container1_name container2_name ...` : stoppe einen oder mehrere laufende Container
- `docker pause container1_name container2_name ...` : pausiere alle Prozesse in einem oder mehreren Containern
- `docker container inspect container1_name container2_name ...` : zeige detaillierte Informationen zu einem oder mehreren Containern an
- `docker export container_name` : exportiere das Dateisystem eines Containers als `.tar` Archiv
- `docker commit container_name` : erstelle ein neues Image aus den Änderungen eines Containers

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/
