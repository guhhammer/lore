# ect

Efficiente Tool di Compressione (o ECT) è un ottimizzatore di file scritto in C++. Supporta file PNG, JPEG, gzip e Zip.

## Usage
- `ect file.png` : comprimi un file
- `ect -9 --mt-deflate file.png` : comprimi un file con il massimo livello di compressione utilizzando più thread
- `ect -keep -recurse percorso/della/directory` : comprimi tutti i file in una directory ricorsivamente, mantenendo la data di modifica originale

## Links
- More information: https://github.com/fhanau/Efficient-Compression-Tool
