# diskutil

Dienstprogramm zur Verwaltung lokaler Festplatten und Volumes.

## Usage
- `diskutil list` : auflisten aller aktuell verfügbaren Festplatten, Partitionen und gemounteten Volumes
- `diskutil repairVolume /dev/diskX` : reparieren der Dateisystem-Datenstrukturen eines Volumes
- `diskutil unmountDisk /dev/diskX` : einen Datenträger aushängen
- `diskutil eject /dev/disk1` : eine CD/DVD auswerfen (zuerst aushängen)

## Links
- More information: https://keith.github.io/xcode-man-pages/diskutil.8.html
