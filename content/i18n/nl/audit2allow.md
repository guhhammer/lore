# audit2allow

Genereer SELinux-beleidsregels op basis van auditlogs. Onderdeel van het `policycoreutils-python-utils` pakket.

## Usage
- `sudo audit2allow -a` : genereer toestemmingsregels op basis van recente auditweigeringen en toon deze
- `sudo audit2allow -i pad/naar/audit.log` : genereer toestemmingsregels op basis van een specifiek auditlogbestand
- `sudo audit2allow -a -M module_naam` : genereer een beleidsmodule op basis van recente auditweigeringen
- `sudo audit2allow -a --why` : leg uit waarom SELinux-weigeringen plaatsvonden (hetzelfde als `audit2why`)
- `sudo audit2allow -a -e` : toon gedetailleerde informatie over gegenereerde berichten
- `sudo audit2allow -a -R` : gebruik geïnstalleerde macro's om een referentiebeleid te genereren
- `sudo ausearch -m avc -c service_naam | audit2allow -M beleid_naam` : genereer toestemmingsregels voor een specifieke service
- `sudo audit2allow -a -v` : schakel uitgebreide uitvoermodus in

## Links
- More information: https://manned.org/audit2allow
