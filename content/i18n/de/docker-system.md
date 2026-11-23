# docker system

Verwalte Docker Daten und zeige systemweite Informationen an.

## Usage
- `docker system` : zeige Hilfe
- `docker system df` : zeige Docker Festplattennutzung
- `docker system df -v` : zeige detaillierte Informationen zur Festplattennutzung
- `docker system prune` : entferne nicht-verwendete Daten
- `docker system prune --filter "until=stundenhminutenm"` : entferne nicht-verwendete Daten, die älter als die angegebene Zeit sind
- `docker system events` : zeige Echtzeit-Events vom Docker Daemon
- `docker system events -f 'type=container' --format '\{\{json .\}\}'` : zeige Echtzeit-Events von Containern und formatiere jede Zeile als gültiges JSON
- `docker system info` : zeige systemweite Informationen

## Links
- More information: https://docs.docker.com/reference/cli/docker/system/
