# docker container stats

Zeige den Ressourcen-Verbrauch von Containern in Echtzeit.

## Usage
- `docker stats` : zeige sich stetig aktualisierende Statistiken von allen laufenden Containern
- `docker stats container_name` : zeige sich stetig aktualisierende Statistiken der durch Leerzeichen getrennten Container
- `docker stats --format "\{\{.Name\}\}:\t\{\{.CPUPerc\}\}"` : ändere das Spaltenformat um die CPU Auslastung von Containern in Prozent anzuzeigen
- `docker stats -a` : zeige Statistiken für alle Container (laufende und gestoppte)
- `docker stats --no-stream` : deaktiviere die laufende Aktualisierung und zeige nur die aktuellen Statistiken

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/stats/
