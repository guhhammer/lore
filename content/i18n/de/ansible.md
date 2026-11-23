# ansible

Verwalte Computergruppen per Fernzugriff über SSH (Verwende die Datei `/etc/ansible/hosts`, um neue Gruppen/Hosts hinzuzufügen). Manche Unterbefehle wie `galaxy` sind separat dokumentiert.

## Usage
- `ansible gruppe --list-hosts` : liste Hosts auf, die zu einer Gruppe gehören
- `ansible gruppe -m ping` : pinge eine Gruppe von Hosts an
- `ansible gruppe -m setup` : zeige Informationen über eine Gruppe von Hosts an
- `ansible gruppe -m command -a 'befehl'` : führe einen Befehl auf einer Gruppe von Hosts aus
- `ansible gruppe -b --ask-become-pass -m command -a 'befehl'` : führe einen Befehl mit administrativen Privilegien aus
- `ansible Gruppe -i inventardatei -m command -a 'befehl'` : führe einen Befehl mit einer benutzerdefinierten Inventardatei aus
- `ansible localhost -m debug -a 'var=groups.keys()'` : liste alle Gruppen eines Inventars auf

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible.html
