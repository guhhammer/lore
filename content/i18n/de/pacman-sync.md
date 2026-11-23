# pacman-sync

Arch Linux Paketverwaltungs-Werkzeug.

## Usage
- `sudo pacman --sync paketname` : installiere ein neues Paket
- `sudo pacman --sync --refresh --sysupgrade` : synchronisiere und aktualisiere alle Pakete (füge `--downloadonly` hinzu um die Pakete nur herunterzuladen und nicht zu aktualisieren)
- `sudo pacman --sync --refresh --sysupgrade --noconfirm paketname` : aktualisiere alle Pakete und installiere ein neues ohne Bestätigungsaufforderung
- `pacman --sync --search "suchmuster"` : suche in der Paketdatenbank mit einem regulären Ausdruck oder Schlüsselwort
- `pacman --sync --info paketname` : zeige Informationen über ein Paket an
- `sudo pacman --sync --refresh --sysupgrade --overwrite pfad/zu/datei` : überschreibe widersprüchliche Dateien während einer Paketaktualisierung
- `sudo pacman --sync --refresh --sysupgrade --ignore paketname` : synchronisiere und aktualisiere alle Pakete, ignoriere aber ein bestimmtes Paket (kann mehr als einmal angegeben werden)
- `sudo pacman --sync --clean` : entferne nicht installierte Pakete und ungenutzte Repositorys vom Cache (nutze zwei `--clean` Operationen um alle Pakete aufzuräumen)

## Links
- More information: https://manned.org/pacman.8
