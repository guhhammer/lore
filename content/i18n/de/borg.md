# borg

Deduplizierendes Sicherungswerkzeug. Erstellt lokale oder entfernte Sicherungen, die als Dateisysteme einhängbar sind.

## Usage
- `borg init -e repokey|keyfile|none|... pfad/zu/repo_verzeichnis` : initialisiere ein lokales Repository
- `borg create --progress pfad/zu/repo_verzeichnis::Montag pfad/zu/quell_verzeichnis` : sichere ein Verzeichnis in das Repository und erstelle ein Archiv mit dem Namen "Montag"
- `borg list pfad/zu/repo_verzeichnis` : liste alle Archive in einem Repository auf
- `borg extract benutzer@host:pfad/zu/repo_verzeichnis::Montag pfad/zu/ziel_verzeichnis --exclude '*.ext'` : extrahiere ein bestimmtes Verzeichnis aus dem "Montag"-Archiv in einem entfernten Repository, unter Ausschluss aller `*.ext`-Dateien
- `borg prune --keep-within 7d --list pfad/zu/repo_verzeichnis` : bereinige ein Repository, indem alle Archive gelöscht werden, die älter als 7 Tage sind und Änderungen aufweisen
- `borg mount pfad/zu/repo_verzeichnis::Montag pfad/zu/mountpoint` : hänge ein Repository als FUSE-Dateisystem ein
- `borg create --help` : zeige Hilfe zur Erstellung von Archiven an

## Links
- More information: https://borgbackup.readthedocs.io/en/stable/usage/general.html
