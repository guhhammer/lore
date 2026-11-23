# docker container cp

Kopiere Dateien oder Verzeichnisse zwischen Host- und Container-Dateisystem.

## Usage
- `docker cp pfad/zu/datei_oder_verzeichnis_auf_host container_name:pfad/zu/datei_oder_verzeichnis_in_container` : kopiere eine Datei oder ein Verzeichnis vom Host in einen Container
- `docker cp container_name:pfad/zu/datei_oder_verzeichnis_in_container pfad/zu/datei_oder_verzeichnis_auf_host` : kopiere eine Datei oder ein Verzeichnis von einem Container zum Host
- `docker cp -L pfad/zu/symlink_auf_host container_name:pfad/zu/datei_oder_verzeichnis_in_container` : kopiere eine Datei oder ein Verzeichnis vom Host in einen Container und folge dabei Symlinks (kopiert die verlinkten Dateien statt der Symlinks)

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/cp/
