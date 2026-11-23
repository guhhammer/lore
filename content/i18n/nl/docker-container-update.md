# docker container update

Configuratie van Docker-containers updaten. Opmerking: dit commando wordt niet ondersteund voor Windows-containers.

## Usage
- `docker update --restart always|no|on-failure|unless-stopped container_naam` : update het herstartbeleid om toe te passen wanneer een specifieke container wordt afgesloten
- `docker update --restart on-failure:3 container_naam` : update het beleid om een specifieke container tot drie keer toe opnieuw te starten als deze afsluit met een exit-status van niet nul
- `docker update --cpus aantal container_naam` : update het aantal CPU's dat beschikbaar is voor een specifieke container
- `docker update -m limietM container_naam` : update het geheugenlimiet in Megabytes voor een specifieke container
- `docker update --pids-limit aantal container_naam` : update het maximumaantal proces-ID's dat is toegestaan in een specifieke container (gebruik `-1` voor onbeperkt)
- `docker update --memory-swap limietM container_naam` : update de hoeveelheid geheugen in Megabytes die een specifieke container naar schijf mag swappen (gebruik `-1` voor onbeperkt)

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/update/
