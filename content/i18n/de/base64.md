# base64

Kodieren oder Dekodieren von Dateien oder Standardeingaben in/aus Base64, zur Standardausgabe.

## Usage
- `base64 pfad/zu/datei` : kodiere den Inhalt einer Datei als Base64 und schreibe das Ergebnis nach `stdout`
- `base64 -w 0|76|... pfad/zu/datei` : wikkel gecodeerde uitvoer in op een specifieke breedte (`0` schakelt inpakken uit)
- `base64 -d pfad/zu/datei` : dekodiere den Inhalt einer Datei als Base64 und schreibe das Ergebnis nach `stdout`
- `befehl | base64` : kodiere von `stdin`
- `befehl | base64 -d` : dekodiere von `stdin`

## Links
- More information: https://manned.org/base64
