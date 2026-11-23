# diskutil

Strumento per gestire i dischi locali e i volumi.

## Usage
- `diskutil list` : mostra tutti i dischi correnti, le partizioni e i volumi montati
- `diskutil repairVolume /dev/diskX` : ripara le strutture dati del filesystem di un volume
- `diskutil unmountDisk /dev/diskX` : smonta un volume
- `diskutil eject /dev/disk1` : estrai un CD/DVD (smontando prima dell'estrazione)

## Links
- More information: https://keith.github.io/xcode-man-pages/diskutil.8.html
