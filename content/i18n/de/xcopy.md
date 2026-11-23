# xcopy

Kopieren von Dateien und Verzeichnisbäumen.

## Usage
- `xcopy pfad/zu/datei_oder_verzeichnis pfad/zu/ziel` : kopiere Datei(en) an den angegebenen Zielort
- `xcopy pfad/zu/datei_oder_verzeichnis pfad/zu/ziel /p` : liste die zu kopierenden Dateien vor dem Kopieren auf
- `xcopy pfad/zu/datei_oder_verzeichnis pfad/zu/ziel /t` : kopiere nur die Verzeichnisstruktur ohne Dateien
- `xcopy pfad/zu/datei_oder_verzeichnis pfad/zu/ziel /e` : kopiere leere Verzeichnisse mit
- `xcopy pfad/zu/datei_oder_verzeichnis pfad/zu/ziel /o` : behalte die Quell-Zugriffsrichtlinien (ACL) im Ziel Verzeichnis bei
- `xcopy pfad/zu/datei_oder_verzeichnis pfad/zu/ziel /z` : setze den Vorgang nach Unterbrechung der Netzwerkverbindung fort
- `xcopy pfad/zu/datei_oder_verzeichnis pfad/zu/ziel /y` : überschreibe bereits vorhandene Zieldateien automatisch
- `xcopy /?` : zeige Hilfe an
