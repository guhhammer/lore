# docker compose

Starte und verwalte Anwendungen, welche aus mehreren Docker Containern bestehen.

## Usage
- `docker compose ps` : liste alle laufenden Container auf
- `docker compose up -d` : erzeuge und starte alle Container im Hintergrund unter der Verwendung der Datei `docker-compose.yml` im aktuellen Verzeichnis
- `docker compose up --build` : starte alle Container. Erzeuge zugehörige Docker Images bei Bedarf neu
- `docker compose -p Projektname -f pfad/zu/verzeichnis up` : starte alle Container durch Angabe eines Projektnamens unter Verwendung einer alternativen Compose-Datei
- `docker compose stop` : stoppe alle laufenden Container
- `docker compose down --rmi all -v` : stoppe und entferne alle Container inklusive zugehöriger Netzwerke, Volumes und Images
- `docker compose logs -f` : zeige die Logs aller Container kontinuierlich an
- `docker compose logs -f container_name` : zeige die Logs eines spezifischen Containers kontinuierlich an

## Links
- More information: https://docs.docker.com/reference/cli/docker/compose/
