# git send-email

Sende eine Menge von Patches als E-Mail. Patches können als Dateien, Ordner oder Liste von Revisionen spezifiziert werden.

## Usage
- `git send-email -1` : sende den letzten Commit des aktuellen Branches
- `git send-email -1 commit` : sende einen spezifischen Commit
- `git send-email -10` : sende die letzten (z.B. 10) Commits des aktuellen Branches
- `git send-email -anzahl_an_commits --compose` : editiere eine E-Mail mit einer Reihe von Patches im Standardmailclienten
- `git send-email -anzahl_an_commits --annotate` : bearbeite den E-Mail Text jedes der zu versendenden Patches

## Links
- More information: https://git-scm.com/docs/git-send-email
