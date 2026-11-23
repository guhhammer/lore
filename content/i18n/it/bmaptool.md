# bmaptool

Crea o copia blockmap intelligentemente (e quindi più velocemente di `cp` o `dd`).

## Usage
- `bmaptool create -o blockmap.bmap sorgente.img` : crea una blockmap da un file immagine
- `bmaptool copy --bmap blockmap.bmap sorgente.img /dev/sdb` : copia un file immagine su sdb
- `bmaptool copy --bmap blockmap.bmap sorgente.img.gz /dev/sdb` : copia un file immagine compresso su sdb
- `bmaptool copy --nobmap sorgente.img /dev/sdb` : copia un file immagine su sdb senza utilizzare una blockmap

## Links
- More information: https://manned.org/bmaptool
