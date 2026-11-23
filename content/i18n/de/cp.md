# cp

Kopiere Dateien und Verzeichnisse.

## Usage
- `cp pfad/zu/datei pfad/zu/kopie` : kopiere eine Datei an einen anderen Ort
- `cp pfad/zu/datei pfad/zu/zielverzeichnis` : kopiere eine Datei an einen anderen Ort und behalte den Dateinamen
- `cp -r pfad/zu/verzeichnis pfad/zu/zielverzeichnis` : kopiere ein Verzeichnis rekursiv (falls der Zielort bereits existiert, wird das Verzeichnis in das Zielverzeichnis kopiert)
- `cp -vr pfad/zu/verzeichnis pfad/zu/zielverzeichnis` : kopiere ein Verzeichnis rekursiv und gib alle Dateien aus, während sie kopiert werden
- `cp -i /pfad/zu/*.txt pfad/zu/zielverzeichnis` : kopiere alle Textdateien in einem Verzeichnis und warte auf eine Bestätigung, falls eine Datei überschrieben werden soll
- `cp -L link pfad/zu/zielverzeichnis` : folge symbolischen Links vor dem Kopieren

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/cp-invocation.html
