# podman-compose

Voer een Compose Specification container-definitie uit en beheer deze.

## Usage
- `podman-compose ps` : toon alle draaiende containers
- `podman-compose up -d` : maak alle containers aan en start ze op de achtergrond met een lokaal `docker-compose.yml`
- `podman-compose up --build` : start alle containers, en bouw deze indien nodig
- `podman-compose -f pad/naar/bestand.yaml up` : start alle containers met een alternatief compose-bestand
- `podman-compose stop` : stop alle draaiende containers
- `podman-compose down -v` : verwijder alle containers, netwerken en volumes
- `podman-compose logs -f container_naam` : volg de logs van een container (laat alle containernamen weg)
- `podman-compose run service_naam commando` : voer een eenmalig commando uit in een service zonder gekoppelde poorten

## Links
- More information: https://github.com/containers/podman-compose
