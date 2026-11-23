# dmidecode

Mostra la taula de continguts del DMI (també conegut com SMBIOS) en un format llegible per humans. Requereix privilegis de root.

## Usage
- `sudo dmidecode` : mostra tota la taula de continguts del DMI
- `sudo dmidecode -s bios-version` : mostra la versió de la BIOS
- `sudo dmidecode -s system-serial-number` : mostra el número de sèrie del equip
- `sudo dmidecode -t bios` : mostra informació de la BIOS
- `sudo dmidecode -t processor` : mostra informació de la CPU
- `sudo dmidecode -t memory` : mostra informació de la memòria

## Links
- More information: https://manned.org/dmidecode
