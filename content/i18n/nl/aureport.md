# aureport

Genereer samenvattende rapporten van auditd-logs.

## Usage
- `sudo aureport` : toon een samenvatting van auditd-gebeurtenissen
- `sudo aureport -l` : genereer een samenvatting van inloggebeurtenissen
- `sudo aureport -s` : toon alle syscall-rapporten
- `sudo aureport -x` : genereer een samenvatting van gebeurtenissen voor uitvoerbare bestanden
- `sudo aureport -ts starttijd -te eindtijd` : toon een samenvatting van gebeurtenissen voor een specifieke periode
- `sudo aureport -t` : toon alle auditbestanden en de periode van gebeurtenissen die ze bestrijken
- `aureport --help` : toon de help

## Links
- More information: https://manned.org/aureport
