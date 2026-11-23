# pfctl

Beheer het pakketfilterapparaat.

## Usage
- `sudo pfctl -e` : schakel het pakketfilter in
- `sudo pfctl -d` : schakel het pakketfilter uit
- `sudo pfctl -f pad/naar/pf.conf` : laad regels uit een configuratiebestand
- `pfctl -sr` : toon alle actieve regels
- `pfctl -s info` : toon statusinformatie van het pakketfilter
- `sudo pfctl -F rules` : verwijder alle regels

## Links
- More information: https://man.freebsd.org/cgi/man.cgi?query=pfctl
