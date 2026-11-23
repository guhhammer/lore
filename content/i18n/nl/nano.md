# nano

Tekstbewerker. Een verbeterde `pico`-kloon.

## Usage
- `nano pad/naar/bestand1 pad/naar/bestand2 ...` : open specifieke bestanden, ga naar het volgende bestand bij het sluiten van de vorige
- `nano -I` : start de tekstbewerker zonder gebruik te maken van configuratiebestanden
- `nano +regel,kolom pad/naar/bestand` : open een bestand en positioneer de cursor op een specifieke regel en kolom
- `nano -S pad/naar/bestand` : open een bestand en zet 'soft wrapping' aan
- `nano -i pad/naar/bestand` : open een bestand en spring nieuwe regels in volgens de inspringing van de vorige regel
- `nano -B pad/naar/bestand` : open een bestand en maak een reservekopie (`pad/naar/bestand~`) bij het opslaan
- `nano -R pad/naar/bestand` : open een bestand in de beperkte modus (d.w.z. lees/schrijf niet naar bestanden die niet op de command-line zijn gespecificeerd)
- `<Ctrl x>` : sluit nano

## Links
- More information: https://nano-editor.org/dist/latest/nano.html
