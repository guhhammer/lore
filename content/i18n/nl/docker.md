# docker

Beheer Docker-containers en -images. Sommige subcommando's zoals `container`, `image`, etc. hebben hun eigen documentatie.

## Usage
- `docker ps -a` : toon alle Docker-containers (actief en gestopt)
- `docker run --name container_naam image` : start een container van een image, met een aangepaste naam
- `docker container start|stop container_naam` : start of stop een bestaande container
- `docker pull image` : download een image uit een Docker register
- `docker images` : toon reeds gedownloade images
- `docker exec -it container_naam sh` : open een interactieve tty met Bourne shell (`sh`) binnen een draaiende container
- `docker rm container1 container2 ...` : verwijder gestopte containers
- `docker logs -f container_naam` : toon de logs van een container en volg deze

## Links
- More information: https://docs.docker.com/reference/cli/docker/
