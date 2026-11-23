# git branch

Verwalte und Arbeite mit Git Branches.

## Usage
- `git branch -a` : liste alle Branches auf (Lokal und Remote). Der momentan aktive (ausgecheckte) Branch wird mit `*` markiert
- `git branch -a --contains commit_hash` : liste alle Branches auf, welche einen spezifischen Git-Commit in ihrer Historie enthalten
- `git branch --show-current` : zeige den Namen des aktuellen Branches
- `git branch branch_name` : erstelle einen neuen Branch auf Basis des letzten Commits
- `git branch branch_name commit_hash` : erstelle einen neuen Branch auf Basis eines bestimmten Commits
- `git branch -m alter_branch_name neuer_branch_name` : benenne einen Branches um (der Branch muss nicht ausgecheckt sein)
- `git branch -d branch_name` : lösche einen lokalen Branch (der Branch muss nicht ausgecheckt sein)
- `git push remote_name -d remote_branch_name` : lösche einen remote-Branch

## Links
- More information: https://git-scm.com/docs/git-branch
