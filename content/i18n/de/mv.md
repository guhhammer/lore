# mv

Verschiebe Dateien oder Verzeichnisse oder benenne diese um.

## Usage
- `mv pfad/zu/datei pfad/zu/zieldatei` : verschiebe eine Datei an einen beliebigen Ort
- `mv pfad/zu/datei1 pfad/zu/datei2 ... pfad/zu/ziel_verzeichnis` : verschiebe mehrere Dateien in ein anderes Verzeichnis und behalte deren Namen bei
- `mv -f pfad/zu/datei pfad/zu/zieldatei` : überschreibe bereits existierende Dateien ohne vorherige Bestätigung
- `mv -i pfad/zu/datei pfad/zu/zieldatei` : überschreibe bereits existierende Dateien nach Bestätigung (unabhängig von Dateirechten)
- `mv -n pfad/zu/datei pfad/zu/zieldatei` : verhindere das Überschreiben existierender Dateien am Zielort
- `mv -v pfad/zu/datei pfad/zu/zieldatei` : liste Dateien und deren Details auf während sie verschoben werden

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/mv-invocation.html
