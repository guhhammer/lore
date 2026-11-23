# git log

Zeigt die Commit-Historie an.

## Usage
- `git log` : zeige die Sequenz der Commits des Git-Repository im aktuellen Verzeichnis, beginnend mit dem aktuellen, an
- `git log -p pfad/zu/datei_oder_verzeichnis` : zeige die Historie einer bestimmten Datei oder eines Verzeichnisses, inklusive Unterschiede, an
- `git log --stat` : zeige einen Überblick der Commits an und welche Dateien jeweils verändert wurden
- `git log --oneline --graph` : zeige einen Graphen von Commits im aktuellen Branch, wobei jeweils nur die erste Zeile der Commit-Nachricht angezeigt wird
- `git log --oneline --decorate --all --graph` : zeige einen Graphen von allen Commits, Tags und Branches im gesamten Repository
- `git log -i --grep text` : zeige nur Commits, deren Commit-Nachricht einen bestimmten Text enthalten (Ohne Beachtung von Groß- und Kleinschreibung)
- `git log -n anzahl --author "autor"` : zeige die letzten N Commits eines bestimmten Autors
- `git log --before "2017-01-29" --after "2017-01-17"` : zeige alle Commits zwischen zwei Zeitpunkten an (yyyy-mm-dd)

## Links
- More information: https://git-scm.com/docs/git-log
