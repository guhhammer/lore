# git apply

Integriere eine Patch-Datei und/oder füge sie zum Index hinzu.

## Usage
- `git apply -v pfad/zu/datei` : gib Informationen über gepatchte Dateien aus
- `git apply --index pfad/zu/datei` : integriere die Patch-Datei und füge sie zum Index hinzu
- `curl -L https://example.com/datei.patch | git apply` : integriere eine externe Patch-Datei
- `git apply --stat --apply pfad/zu/datei` : gib diffstat des Inputs aus und integriere die Patch-Datei
- `git apply -R pfad/zu/datei` : integriere eine Patch-Datei in umgekehrter Reihenfolge
- `git apply --cached pfad/zu/datei` : speichere das Ergebnis einer Patch-Datei im Index, ohne den Arbeitsbaum zu verändern

## Links
- More information: https://git-scm.com/docs/git-apply
