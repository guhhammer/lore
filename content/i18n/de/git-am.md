# git am

Patch-Dateien integrieren. Nützlich beim Empfang von Commits per E-Mail.

## Usage
- `git am pfad/zu/datei.patch` : integriere eine Patch-Datei
- `curl -L https://example.com/file.patch | git am` : herunterladen und Integrieren einer Patch-Datei
- `git am --abort` : brich das Integrieren einer Patch-Datei ab
- `git am --reject pfad/zu/datei.patch` : integriere so viele Patch-Dateien wie möglich und speichere fehlgeschlagene Teile

## Links
- More information: https://git-scm.com/docs/git-am
