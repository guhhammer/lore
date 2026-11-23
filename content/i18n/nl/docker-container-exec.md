# docker container exec

Voer een opdracht uit op een Docker-container die al draait.

## Usage
- `docker exec -it container_naam /bin/bash` : start een interactieve shellsessie op een reeds draaiende container
- `docker exec -d container_naam commando` : voer een commando uit op de achtergrond (detached) op een draaiende container
- `docker exec -it -w pad/naar/map container_naam commando` : selecteer de werkmap waarin een gegeven commando moet worden uitgevoerd
- `docker exec -i -d container_naam commando` : voer een opdracht op de achtergrond uit op een bestaande container, maar houd `stdin` open
- `docker exec -it -e variabele_naam=waarde container_naam /bin/bash` : stel een omgevingsvariabele in in een lopende Bash-sessie
- `docker exec -u gebruiker container_naam commando` : voer een opdracht uit als een specifieke gebruiker

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/exec/
