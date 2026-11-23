# pass

Programm zum Speichern und Lesen von Passwörtern und anderen sensiblen Daten. Die Daten sind mit GPG verschlüsselt und werden mit einem Git repository verwaltet.

## Usage
- `pass init gpg_id_1 gpg_id_2 ...` : initialisiere oder verschlüssle einen neuen oder bestehenden Speicher mit einer oder mehreren GPG IDs neu
- `pass insert -m pfad/zu/datei` : speichere das Passwort und zusätzliche Informationen (`<Ctrl d>` auf neuer Zeile zum abschließen)
- `pass edit pfad/zu/datei` : bearbeite einen bestimmten Eintrag
- `pass -c pfad/zu/datei` : kopiere das Passwort (die erste Zeile des Eintrags) in die Zwischenablage
- `pass` : zeige die Baumstruktur des Passwort-Stores an
- `pass generate -c pfad/zu/datei n` : generiere ein neues, zufälliges Passwort mit Länge n und kopiere is in die Zwischenablage
- `pass git init` : initialisiere ein Git Repository (Alle durch pass durchgeführten Änderungen werden automatisch committed)
- `pass git befehl` : führe einen Git-Befehl für den Passwort-Store aus

## Links
- More information: https://git.zx2c4.com/password-store/about/
