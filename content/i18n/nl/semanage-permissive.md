# semanage permissive

Beheer persistente SELinux permissieve domeinen. Let op dat dit het proces effectief onbeperkt maakt. Voor langdurig gebruik wordt aanbevolen om SELinux correct te configureren.

## Usage
- `sudo semanage permissive -l` : toon alle procestypen (ook wel domeinen genoemd) die in permissieve modus zijn
- `sudo semanage permissive -a httpd_t` : stel de permissieve modus in voor een domein
- `sudo semanage permissive -d httpd_t` : zet de permissieve modus uit voor een domein

## Links
- More information: https://manned.org/semanage-permissive
