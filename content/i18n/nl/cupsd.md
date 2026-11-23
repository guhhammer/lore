# cupsd

Server daemon voor de CUPS print server.

## Usage
- `cupsd` : start `cupsd` op de achtergrond, aka. als een daemon
- `cupsd -f` : start `cupsd` op de voorgrond (f)
- `cupsd -l` : draai (l) `cupsd` op aanvraag (vaak gebruikt door `launchd` of `systemd`)
- `cupsd -c pad/naar/cupsd.conf` : start `cupsd` met het gespecificeerde `cupsd.conf` configuratiebestand
- `cupsd -s pad/naar/cups-files.conf` : start `cupsd` met het gespecificeerde `cups-files.conf` configuratiebestand
- `cupsd -t -c pad/naar/cupsd.conf` : test het `cupsd.conf` configuratiebestand voor fouten
- `cupsd -t -s pad/naar/cups-files.conf` : test het `cups-files.conf` configuratiebestand voor fouten
- `cupsd -h` : toon de help

## Links
- More information: https://openprinting.github.io/cups/doc/man-cupsd.html
