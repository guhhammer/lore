# whereis

Zoek de binary-, bron- en manualbestanden voor een commando.

## Usage
- `whereis ssh` : zoek binary, bron en man-pagina's voor SSH
- `whereis -bm ls` : zoek binary en man-pagina's voor ls
- `whereis -s gcc -m git` : zoek source van gcc en man-pagina's voor Git
- `whereis -b -B /usr/bin/ -f gcc` : zoek binaries voor gcc alleen in `/usr/bin/`
- `whereis -u *` : zoek ongewone (u) binaries (die meer of minder dan één binary op het systeem hebben)
- `whereis -u -m *` : zoek binaries met ongewone (u) manual-vermeldingen (binaries die meer of minder dan één manual geïnstalleerd hebben)

## Links
- More information: https://manned.org/whereis
