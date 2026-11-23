# pacman-query

Arch Linux Paketverwaltungs-Werkzeug.

## Usage
- `pacman -Q` : liste alle installierten Pakete und dessen Versionen auf
- `pacman -Qe` : liste alle ausdrücklich installierten Pakete und dessen Versionen auf
- `pacman -Qo dateiname` : finde heraus welches Paket eine Datei besitzt
- `pacman -Qi paketname` : zeige Informationen über ein installiertes Paket an
- `pacman -Ql paketname` : liste alle Dateien auf welche einem Paket gehören
- `pacman -Qdtq` : liste verwaiste Pakete auf (Pakete welche als Abhängigkeit installiert wurden, aber von keinem Paket benötigt werden)
- `pacman -Qm` : liste installierte Pakete auf welche nicht in den Repositorien gefunden werden können
- `pacman -Qu` : liste veraltete Pakete auf

## Links
- More information: https://manned.org/pacman.8
