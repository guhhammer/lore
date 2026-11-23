# lp

Druckt Dateien.

## Usage
- `echo "test" | lp` : drucke die Ausgabe eines Befehls mit dem Standard-Drucker (siehe `lpstat`)
- `lp pfad/zu/datei` : drucke eine Datei mit dem Standard-Drucker
- `lp -d druckername pfad/zu/datei` : drucke eine Datei mit einem bestimmten Drucker (siehe `lpstat`)
- `lp -n N pfad/zu/datei` : drucke N Kopien einer Datei mit dem Standarddrucker (wobei N die Anzahl gewünschter Kopien ist)
- `lp -P 1,3-5,16 pfad/zu/datei` : drucke nur bestimmte Seiten mit dem Standarddrucker (drucke Seiten 1, 3-5 und 16)
- `lp -i job_id -H resume` : führe einen aufgehaltenen Druckauftrag durch

## Links
- More information: https://manned.org/lp
