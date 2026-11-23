# docker container stats

Toon een livestream van statistieken over het gebruik van bronnen voor containers.

## Usage
- `docker stats` : toon een livestream van de statistieken van alle draaiende containers
- `docker stats container1 container2 ...` : toon een livestream van statistieken voor één of meer containers
- `docker stats --format "\{\{.Name\}\}:\t\{\{.CPUPerc\}\}"` : wijzig het kolommenformaat om het CPU-gebruikspercentage van de container te tonen
- `docker stats -a` : toon statistieken voor alle containers (zowel draaiende als gestopte)
- `docker stats --no-stream` : schakel streaming statistieken uit en haal alleen de huidige statistieken op

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/stats/
