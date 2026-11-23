# numfmt

Converteer getallen naar en van voor mensen leesbare tekenreeksen.

## Usage
- `numfmt --from si 1.5K` : converteer 1.5K (SI-eenheden) naar 1500
- `numfmt --to si 1500` : converteer 1500 naar 1.5K (SI-eenheden)
- `numfmt --from iec 1.5K` : converteer 1.5K (IEC-eenheden) naar 1536
- `numfmt --from auto 1.5Ki` : gebruik de juiste conversie op basis van het achtervoegsel
- `ls -l | numfmt --header=1 --field 5 --to iec` : converteer het 5e veld (1-gebaseerd) naar IEC-eenheden zonder de koptekst te converteren
- `du -s * | numfmt --to iec --format "%-5f"` : converteer naar IEC-eenheden, vul aan met 5 tekens, links uitgelijnd
