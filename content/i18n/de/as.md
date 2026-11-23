# as

Portabler GNU assembler. Hauptsächlich beabsichtigt um output von `gcc` für `ld` vorzubereiten.

## Usage
- `as pfad/zu/datei.s` : assemble eine Datei und schreibe den Output in eine in `a.out`
- `as pfad/zu/datei.s -o pfad/zu/out.o` : assemble den Output einer gegebenen Datei
- `as -f pfad/zu/datei.s` : generiere den Output schneller indem Leerzeichen und Kommentare nicht verarbeitet werden. (Sollte nur für vertrauenswürdige Compiler benutzt werden)
- `as -I pfad/zu/verzeichnis pfad/zu/datei.s` : inkludiere einen gegebenen Pfad in der Liste von Verzeichnissen für die Suche nach Dateien

## Links
- More information: https://manned.org/as
