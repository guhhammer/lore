# sync

Schrijft alle hangende schrijfoperaties naar de juiste schijven.

## Usage
- `sync` : schrijf alle hangende schrijfoperaties naar alle schijven
- `sync pad/naar/bestand` : schrijf alle hangende schrijfoperaties van een enkel bestand naar de schijf
- `sync; echo 3 | sudo tee /proc/sys/vm/drop_caches` : schrijf alle schrijfoperaties en verwijder caches van het bestandssysteem

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/sync-invocation.html
