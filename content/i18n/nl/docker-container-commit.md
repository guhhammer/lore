# docker container commit

Maak een nieuw image van de wijzigingen van een container.

## Usage
- `docker commit container image:tag` : maak een image van een specifieke container
- `docker commit -c "CMD commando" container image:tag` : pas een `CMD` Dockerfile-instructie toe op de aangemaakte image
- `docker commit -c "ENV naam=waarde" container image:tag` : pas een `ENV` Dockerfile-instructie toe op de aangemaakte image
- `docker commit -a "auteur" container image:tag` : maak een image met een specifieke auteur in de metadata
- `docker commit -m "opmerking" container image:tag` : maak een image met een specifieke opmerking in de metadata
- `docker commit -p false container image:tag` : maak een image zonder de container te pauzeren tijdens het vastleggen
- `docker commit --help` : toon de help

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/commit/
