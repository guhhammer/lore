# apt-get

Debian und Ubuntu Paket Management Tool. Suche mit `apt-cache` nach Paketen.

## Usage
- `sudo apt-get update` : aktualisiere die Liste der Paketquellen (es wird empfohlen diesen Befehl zu Beginn auszuführen)
- `sudo apt-get install paket` : installiere ein Paket oder aktualisiere es zur neuesten Version
- `sudo apt-get remove paket` : entferne ein Paket
- `sudo apt-get purge paket` : entferne ein Paket und die dazugehörigen Konfigurationen
- `sudo apt-get upgrade` : aktualisiere alle Pakete auf die neueste Version
- `sudo apt-get autoclean` : reinige das Repository
- `sudo apt-get autoremove` : entferne alle Pakete, die nicht mehr benötigt werden
- `sudo apt-get dist-upgrade` : aktualisiere alle Pakete (wie `upgrade`), aber entfernt alle obsoleten Pakete

## Links
- More information: https://manned.org/apt-get.8
