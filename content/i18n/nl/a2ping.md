# a2ping

Converteer afbeeldingen in EPS- of PDF-bestanden.

## Usage
- `a2ping pad/naar/afbeelding.ext pad/naar/uitvoer.pdf` : converteer een afbeelding naar PDF (Let op: het opgeven van een uitvoerbestandsnaam is optioneel)
- `a2ping --nocompress none|zip|best|flate pad/naar/bestand` : comprimeer het document met behulp van de opgegeven methode
- `a2ping --nohires pad/naar/bestand` : scan HiResBoundingBox indien aanwezig (Let op: de standaard is ja)
- `a2ping --below pad/naar/bestand` : sta pagina-inhoud onder en links van de oorsprong toe (Let op: de standaard is nee)
- `a2ping --gsextra argumenten pad/naar/bestand` : geef extra argumenten door aan `gs`
- `a2ping --extra argumenten pad/naar/bestand` : geef extra argumenten mee aan het externe programma (bijv. `pdftops`)
- `a2ping -h` : toon de help

## Links
- More information: https://manned.org/a2ping
