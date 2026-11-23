# git format-patch

Erstelle `.patch` Dateien. Ermöglicht das Senden von Commits per E-Mail.

## Usage
- `git format-patch origin` : erzeuge eine `.patch` Datei aus allen nicht gepushten Commits
- `git format-patch revision_1..revision_2` : erstelle eine `.patch` Datei aus allen Commits zwischen den angegebenen Revisionen und schreibe diese nach `stdout`
- `git format-patch -3` : erstelle eine `.patch` Datei aus den letzten 3 Commits

## Links
- More information: https://git-scm.com/docs/git-format-patch
