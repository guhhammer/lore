# apt-mark

Tool um den Status eines installierten Paketes zu verändern.

## Usage
- `sudo apt-mark auto paketname` : markiere ein Paket als automatisch installiert
- `sudo apt-mark hold paketname` : halte ein Paket auf seiner aktuellen Version und verhindere dass es aktualisiert wird
- `sudo apt-mark unhold paketname` : erlaube dass ein Paket wieder aktualisiert werden darf
- `apt-mark showmanual` : zeige manuell installierte Pakete
- `apt-mark showhold` : zeige gehaltene Pakete die nicht aktualisiert werden dürfen

## Links
- More information: https://manned.org/apt-mark
