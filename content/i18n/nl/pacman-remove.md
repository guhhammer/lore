# pacman-remove

Verwijder pakketten van het systeem.

## Usage
- `sudo pacman -Rs pakket` : verwijdeR een pakket en zijn afhankelijkheden recursief
- `sudo pacman -Rsn pakket` : verwijdeR een pakket en zijn afhankelijkheden. Maak ook geen back-ups van configuratiebestanden
- `sudo pacman -R --noconfirm pakket` : verwijdeR een pakket zonder bevestigingsprompt
- `sudo pacman -Rsn $(pacman -Qdtq)` : verwijdeR weespakketten (geïnstalleerd als afhankelijkheden maar niet vereist door een ander pakket)
- `sudo pacman -Rc pakket` : verwijdeR een pakket en cascadeer dat naar alle pakketten die ervan afhankelijk zijn
- `pacman -Rp pakket` : toon en print pakketten die beïnvloed zouden worden (verwijdert R geen pakketten)
- `pacman -Rh` : toon de help

## Links
- More information: https://manned.org/pacman.8
