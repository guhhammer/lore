# git commit

Committe Dateien in ein Repository.

## Usage
- `git commit -m "nachricht"` : committe die gestagten Dateien mit einer Nachricht in das Repository
- `git commit -F pfad/zu/commit_nachricht_datei` : committe alle gestagten Dateien zum Repository mit einer Nachricht aus einer Datei
- `git commit -a -m "nachricht"` : stage alle modifizierten Dateien und committe sie mit einer Nachricht
- `git commit -S key_id -m "nachricht"` : committe gestagten Dateien und signiere sie mit dem definierten GPG Schlüssel (oder mit dem in der Konfigurationsdatei definierten, wenn kein Argument angegeben ist)
- `git commit --amend` : ersetze den letzten Commit mit den gerade auf dem Stage liegenden Änderungen
- `git commit pfad/zu/datei1 pfad/zu/datei2 ...` : comitte nur spezifische Dateien (die Dateien müssen schon auf dem Stage liegen)
- `git commit -m "nachricht" --allow-empty` : erzeuge einen Commit, auch wenn keine Dateien auf dem Stage liegen

## Links
- More information: https://git-scm.com/docs/git-commit
