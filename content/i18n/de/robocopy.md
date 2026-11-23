# robocopy

Robustes Kopieren von Dateien und Ordnern. Standardmäßig werden Dateien nur kopiert, wenn die Quell- und Zieldatei ein unterschiedliches Änderungsdatum oder eine unterschiedliche Dateigröße haben.

## Usage
- `robocopy pfad/zu/quelle pfad/zu/ziel *.jpg *.bmp` : alle `.jpg` und `.bmp` Dateien aus dem einen Verzeichnis in ein anderes Verzeichnis kopieren
- `robocopy pfad/zu/quelle pfad/zu/ziel /E` : alle Dateien und Unterverzeichnisse kopieren, einschließlich der leeren Verzeichnisse
- `robocopy pfad/zu/quelle pfad/zu/ziel /MIR /COPYALL` : ein Verzeichnis spiegeln/synchronisieren. Dabei wird Alles, was nicht in der Quelle vorhanden ist, gelöscht sowie alle Attribute und Berechtigungen übertragen
- `robocopy pfad/zu/quelle pfad/zu/ziel /E /XO` : alle Dateien und Unterverzeichnisse kopieren, ausgenommen der Quelldateien, die älter sind als die vorhandenen Zieldateien
- `robocopy pfad/zu/quelle pfad/zu/ziel /MIN:52428800 /L` : gibt alle Dateien aus, die 50 MB und größer sind, anstatt sie zu kopieren
- `robocopy pfad/zu/quelle pfad/zu/ziel /Z /R:5 /W:15` : erlaubt das Fortsetzen des Vorgangs bei Netzwerkverlust, begrenzt die Anzahl an Versuchen auf 5 und die Wartezeit zwischen Versuchen auf 15 Sekunden
- `robocopy /?` : zeige Hilfe an
