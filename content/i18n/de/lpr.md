# lpr

CUPS-Programm zum Drucken von Dateien.

## Usage
- `lpr pfad/zu/datei` : drucke eine Datei mit dem Standarddrucker
- `lpr -# 2 pfad/zu/datei` : drucke 2 Kopien einer Datei
- `lpr -P druckername pfad/zu/datei` : drucke eine Datei mit einem bestimmten Drucker
- `lpr -o page-ranges=2|2-16 pfad/zu/datei` : drucke entweder eine einzelne Seite (z. B. 2) oder mehrere Seiten (z. B. 2-16)
- `lpr -o sides=two-sided-long-edge|two-sided-short-edge pfad/zu/datei` : drucke doppelseitig entweder gespiegelt an der langen oder an der kurzen Seite
- `lpr -o media=a4|letter|legal pfad/zu/datei` : drucke mit festgelegter Papiergröße (je nach Drucker-Konfiguration gibt es mehr Optionen)
- `lpr -o number-up=2|4|6|9|16 pfad/zu/datei` : drucke mehrere Seiten pro Blatt

## Links
- More information: https://openprinting.github.io/cups/doc/man-lpr.html
