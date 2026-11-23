# docker network

Erzeuge und verwalte Docker Netzwerke.

## Usage
- `docker network ls` : liste alle verfügbaren und konfigurierten Docker Netzwerke auf
- `docker network create -d treiber_name netzwerk_name` : erzeuge ein benutzerdefiniertes Netzwerk
- `docker network inspect netzwerk_name` : zeige detaillierte Informationen der mit Leerzeichen separierten Netzwerke an
- `docker network connect netzwerk_name container_name|id` : verbinde einen Container mit einem Netzwerk anhand des Namens oder der ID
- `docker network disconnect netzwerk_name container_name|id` : trenne einen Container von einem Netzwerk
- `docker network prune` : entferne alle unbenutzten (nicht von Containern referenzierten) Netzwerke
- `docker network rm netzwerk_name` : entferne mehrere - durch Leerzeichen getrennte - Netzwerke

## Links
- More information: https://docs.docker.com/reference/cli/docker/network/
