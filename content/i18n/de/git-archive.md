# git archive

Erstelle ein Archiv von Dateien.

## Usage
- `git archive -v HEAD` : erstelle ein `.tar`-Archiv aus dem Inhalt des aktuellen HEAD und gib dieses aus
- `git archive -v --format zip HEAD` : erstelle ein zip-Archiv aus dem Inhalt des aktuellen HEAD und gib dieses aus
- `git archive -v -o pfad/zu/datei.zip HEAD` : erstelle ein zip-Archiv aus dem Inhalt des aktuellen HEAD und speichere dieses in eine Datei
- `git archive -o pfad/zu/datei.tar branch_name` : erstelle ein `.tar`-Archiv aus dem Inhalt des letzten Commits eines bestimmten Branches
- `git archive -o pfad/zu/datei.tar HEAD:pfad/zu/verzeichnis` : erstelle ein tar-Archiv aus dem Inhalt eines bestimmten Verzeichnisses
- `git archive -o pfad/zu/datei.tar --prefix pfad/zu/verzeichnis/ HEAD` : stelle jeder Datei einen Pfad voran, um sie in einem bestimmten Verzeichnis zu archivieren

## Links
- More information: https://git-scm.com/docs/git-archive
