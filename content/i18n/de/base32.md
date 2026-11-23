# base32

Kodieren oder Dekodieren von Dateien oder Standardeingaben in/aus Base32, zur Standardausgabe.

## Usage
- `base32 pfad/zu/datei` : kodiere den Inhalt einer Datei als Base32 und schreibe das Ergebnis nach `stdout`
- `base32 -w 0|76|... pfad/zu/datei` : wikkel gecodeerde uitvoer in op een specifieke breedte (`0` schakelt inpakken uit)
- `base32 -d pfad/zu/datei` : dekodiere den Inhalt einer Datei als Base32 und schreibe das Ergebnis nach `stdout`
- `befehl | base32` : kodiere von `stdin`
- `befehl | base32 -d` : dekodiere von `stdin`

## Links
- More information: https://manned.org/base32
