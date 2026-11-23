# podman

Eenvoudige beheertool voor pods, containers en images. Biedt een met Docker-CLI vergelijkbare command-line. Simpel gezegd: `alias docker=podman`.

## Usage
- `podman ps -a` : toon alle containers (actief en gestopt)
- `podman run --name container_naam image` : maak een container aan van een image, met een aangepaste naam
- `podman start|stop container_naam` : start of stop een bestaande container
- `podman pull image` : download een image uit een register (standaard Docker Hub)
- `podman images` : toon de lijst van reeds gedownloade images
- `podman exec -it container_naam sh` : open een shell binnen een reeds draaiende container
- `podman rm container_naam` : verwijder een gestopte container
- `podman logs -f container_naam_of_id1 container_naam_of_id2 ...` : toon de logs van een of meer containers en volg de log-uitvoer

## Links
- More information: https://docs.podman.io/en/latest/Commands.html
