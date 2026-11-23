# pacman-database

Mit der Arch Linux Paketdatenbank arbeiten. Verschiedene Attribute von installierten Paketen bearbeiten.

## Usage
- `sudo pacman -D --asdeps paketname` : markiere ein Paket als implizit installiert
- `sudo pacman -D --asexplicit paketname` : markiere ein Paket als explizit installiert
- `pacman -Dk` : überprüfe, dass alle Paketabhängigkeiten installiert sind
- `pacman -Dkk` : überprüfe in den Repositorien, dass alle angegebenen Abhängigkeiten verfügbar sind
- `pacman -Dkq` : zeige nur Fehlermeldungen
- `pacman -Dh` : zeige Hilfe an

## Links
- More information: https://manned.org/pacman.8
