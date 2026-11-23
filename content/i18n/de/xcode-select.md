# xcode-select

Wechsel zwischen verschiedenen Xcode Versionen und den enthaltenen Entwicklertools. Wird auch verwendet, um den Pfad zu Xcode zu aktualisieren, wenn dieser sich nach einer Installation geändert hat.

## Usage
- `xcode-select --install` : installiere die Xcode Entwicklertools
- `xcode-select --switch pfad/zu/Xcode.app/Contents/Developer` : wähle einen bestimmten Pfad als aktives Entwicklerverzeichnis aus
- `xcode-select --switch pfad/zu/Xcode.app` : wähle eine Xcode Version aus und ändere das aktive Entwicklerverzeichnis dahin
- `xcode-select --print-path` : gib das derzeit aktive Entwicklerverzeichnis aus
- `sudo xcode-select --reset` : verwerfe alle vom Benutzer angegebenen Entwicklerverzeichnisse (fortan wird der Standardsuchmechanismus verwendet, um diese zu finden)

## Links
- More information: https://developer.apple.com/library/archive/technotes/tn2339/_index.html
