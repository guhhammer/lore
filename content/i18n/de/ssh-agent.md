# ssh-agent

Erstelle einen SSH Agenten-Prozess. Ein SSH Agent behält die hinzugefügten SSH Schlüssel solange verschlüsselt im Arbeitsspeicher, bis diese entfernt werden oder der Agenten-Prozess beendet wird.

## Usage
- `eval $(ssh-agent)` : starte einen SSH Agenten-Prozesses für die aktuelle Shell
- `ssh-agent -k` : beende den aktuell laufenden SSH Agenten-Prozesses

## Links
- More information: https://man.openbsd.org/ssh-agent
