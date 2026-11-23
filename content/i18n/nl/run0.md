# run0

Verhoog rechten interactief. Vergelijkbaar met `sudo`, maar het is geen SUID-binary, authenticatie verloopt via polkit, en commando's worden aangeroepen vanuit een `systemd`-service.

## Usage
- `run0 commando` : voer een commando uit als root
- `run0 -u gebruikersnaam|gebruikers_id -g groepsnaam|groeps_id commando` : voer een commando uit als een andere gebruiker en/of groep

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/run0.html
