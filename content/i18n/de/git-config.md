# git config

Verwalten von benutzerdefinierten Optionen für Git Repositories. Diese Optionen können lokal (für das aktiven Repository) or global (für den aktiven Benutzer) sein.

## Usage
- `git config -l --local` : liste nur lokale Konfigurationseinträge (gespeichert unter `.git/config` im aktiven Repository) auf
- `git config -l --global` : liste nur globale Konfigurationseinträge (gespeichert unter `~/.gitconfig`) auf
- `git config -l --system --show-origin` : liste nur System-Konfigurationseinträge (gespeichert unter `/etc/gitconfig`) und deren Speicherort auf
- `git config alias.unstage` : gib den Wert eines bestimmten Konfigurationseintrags aus
- `git config --global alias.unstage "reset HEAD --"` : setze den globalen Wert eines bestimmten Konfigurationseintrags
- `git config --global --unset alias.unstage` : setze den globalen Wert eines bestimmten Konfigurationseintrags auf seinen Standardwert zurück
- `git config -e` : bearbeite die Git-Konfiguration für das aktuelle Repository mit dem Standard-Editor
- `git config --global -e` : bearbeite die globale Git-Konfiguration mit dem Standard-Editor

## Links
- More information: https://git-scm.com/docs/git-config
