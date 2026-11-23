# nodemon

Beobachtet Dateien und startet Node Applikationen automatisch neu, wenn Änderungen erkannt wurden.

## Usage
- `nodemon pfad/zu/datei.js` : führe die angegebene Datei aus und warte auf Änderungen
- `rs` : manueller Neustart von Nodemon (beachte, dass Nodemon dabei aktiv sein muss)
- `nodemon -i pfad/zu/datei_oder_verzeichnis` : ignoriere bestimmte Dateien
- `nodemon pfad/zu/datei.js argumente` : übergib Argumente an die Node Applikation
- `nodemon argumente pfad/zu/datei.js` : übergib Argumente an node selbst, wenn sie nicht bereits Argumente von nodemon sind (z.B. `--inspect`)
- `nodemon -x "befehl_um_skript_auszuführen argumente" pfad/zu/skript` : führe Nicht-Node Skripte aus
- `nodemon -x "python argumente" pfad/zu/datei.py` : führe ein Python-Skript aus

## Links
- More information: https://github.com/remy/nodemon/tree/main/doc/cli
