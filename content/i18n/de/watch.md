# watch

Führe einen Befehl wiederholt aus und überwache die Ausgabe im Vollbildmodus.

## Usage
- `watch ls` : überwache die Dateien im aktuellen Verzeichnis
- `watch -d df` : überwache verfügbaren Festplatten-Speicherplatz und hebe die Änderungen hervor
- `watch -n 3 "ps aux | grep node"` : überwache "node"-Prozesse und aktualisiere alle 3 Sekunden
- `watch -g df` : überwache, ob sich der Festplatten-Speicherplatz ändert und höre mit der Überwachung auf, sobald eine Änderung auftritt

## Links
- More information: https://manned.org/watch
