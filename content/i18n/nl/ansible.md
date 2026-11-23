# ansible

Beheer groepen van computers op afstand over SSH. (Gebruik het `/etc/ansible/hosts` bestand om nieuwe groepen/hosts toe te voegen). Sommige subcommando's zoals `galaxy` hebben hun eigen documentatie.

## Usage
- `ansible groep --list-hosts` : toon de hosts die tot een groep behoren
- `ansible groep -m ping` : ping een groep met hosts, met gebruik van de ping module
- `ansible groep -m setup` : toon feiten van een groep met hosts, met gebruik van de setup module
- `ansible groep -m command -a 'mijn_commando'` : voer een commando op een groep met hosts uit, met gebruik van de command module met argumenten
- `ansible groep -b --ask-become-pass -m command -a 'mijn_commando'` : voer een commando uit met administratieve rechten
- `ansible groep -i inventaris_bestand -m command -a 'mijn_commando'` : voer een commando uit met een aangepast inventarisbestand
- `ansible localhost -m debug -a 'var=groups.keys()'` : toon de groepen in een inventaris

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible.html
