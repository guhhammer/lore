# audit2why

Leg SELinux-weigeringen van auditlogs uit. Onderdeel van het `policycoreutils-python-utils` pakket.

## Usage
- `sudo audit2why` : leg de nieuwste SELinux-weigering uit
- `sudo audit2why -i pad/naar/audit.log` : leg SELinux-weigeringen van een specifiek auditlogbestand uit
- `sudo ausearch -m avc | audit2why` : leg alle SELinux-weigeringen van een auditlog uit
- `sudo ausearch -m avc -c service_naam | audit2why` : leg weigeringen van een specifieke service uit

## Links
- More information: https://manned.org/audit2why
