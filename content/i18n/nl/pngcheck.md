# pngcheck

Forensics tool voor het valideren van de integriteit van PNG-gebaseerde (PNG, JNG, MNG) afbeeldingsbestanden. Kan ook embedded afbeeldingen en tekst van een bestand extraheren.

## Usage
- `pngcheck pad/naar/afbeelding.png` : verifieer de integriteit van een afbeeldingsbestand (breedte, hoogte, en kleurdiepte)
- `pngcheck -c pad/naar/afbeelding.png` : toon informatie over een afbeelding met gekleurde (c) uitvoer
- `pngcheck -cvt pad/naar/afbeelding.png` : toon gedetailleerde (v) informatie over een afbeelding
- `cat pad/naar/afbeelding.png | pngcheck -cvt` : ontvang een afbeelding van `stdin` en toon gedetailleerde informatie
- `pngcheck -s pad/naar/afbeelding.png` : zoek (s) naar PNG-bestanden binnen een specifiek bestand en toon informatie
- `pngcheck -x pad/naar/afbeelding.png` : zoek naar PNG's binnen een ander bestand en extracteer ze

## Links
- More information: https://manned.org/pngcheck
