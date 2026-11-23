# docker service

Verwalte Docker Services.

## Usage
- `docker service ls` : liste alle Services auf
- `docker service create --name service_name image:tag` : erstelle einen neuen Service
- `docker service inspect service_name|id` : zeige detaillierte Informationen der mit Leerzeichen separierten Services an
- `docker service ps service_name|id` : liste die Tasks der mit Leerzeichen separierten Services auf
- `docker service scale service_name=anzahl_an_replikaten` : skaliere die angegebenen Services auf eine bestimmte Anzahl an Replikaten
- `docker service rm service_name|id` : lösche die mit Leerzeichen separierten Services

## Links
- More information: https://docs.docker.com/reference/cli/docker/service/
