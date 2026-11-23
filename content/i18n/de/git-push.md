# git push

Lade Commits in ein Remote-Repository hoch.

## Usage
- `git push` : sende lokale Änderungen des aktuellen Branches zu seinem entfernten Repository (Remote Branch)
- `git push remote_name lokaler_branch` : sende lokale Änderungen des angegebenen Branches zu seinem entfernten Repository
- `git push -u remote_name remote_branch` : lade den aktuellen Branch in ein entferntes Repository mit Angabe des Namens des entfernten Branches hoch
- `git push remote_name lokaler_branch:entfernter_branch` : lade Änderungen eines bestimmten lokalen Branches zu einem bestimmten entfernten Branch hoch
- `git push --all remote_name` : lade Änderungen aller lokalen Branches zu ihrem entfernten Repository hoch
- `git push remote_name -d remote_branch` : lösche einen Branch in einem entfernten Repository
- `git push --prune remote_name` : entferne alle remote Branches, welche kein lokales Gegenstück besitzen
- `git push --tags` : veröffentliche Tags, welche noch nicht im entfernten Repository vorhanden sind

## Links
- More information: https://git-scm.com/docs/git-push
