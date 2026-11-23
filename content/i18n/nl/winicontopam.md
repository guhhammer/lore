# winicontopam

Converteer een Windows ICO bestand naar een PAM bestand.

## Usage
- `winicontopam pad/naar/invoer_bestand.ico >pad/naar/uitvoer.pam` : lees een ICO bestand en converteer de beste kwaliteit afbeelding daarin naar het PAM formaat
- `winicontopam -al pad/naar/invoer_bestand.ico >pad/naar/uitvoer.pam` : converteer alle afbeeldingen in het invoerbestand naar PAM
- `winicontopam -i n pad/naar/invoer_bestand.ico >pad/naar/uitvoer.pam` : converteer de n-de afbeelding in het invoerbestand naar PAM
- `winicontopam -an pad/naar/invoer_bestand.ico >pad/naar/uitvoer.pam` : als de afbeelding(en) die geëxtraheerd moeten worden gegradeerde transparantiegegevens en een AND mask bevatten, schrijf de AND mask naar het vijfde kanaal van het PAM-uitvoerbestand

## Links
- More information: https://netpbm.sourceforge.net/doc/winicontopam.html
