# pacman

Hulpprogramma voor het beheren van pakketten op Arch Linux.

## Usage
- `sudo pacman -Syu` : synchroniseer en update alle pakketten
- `sudo pacman -S pakket` : installeer een nieuw pakket
- `sudo pacman -Rs pakket` : verwijdeR een pakket en zijn afhankelijkheden
- `pacman -Ss "zoekterm"` : doorzoek (s) de pakketdatabase met een `regex` of zoekwoord
- `pacman -F "bestandsnaam"` : zoek in de database voor pakketten die een specifiek bestand (F) bevatten
- `pacman -Qe` : toon alleen de expliciet geïnstalleerde pakketten en versies
- `pacman -Qtdq` : toon weespakketten (geïnstalleerd als afhankelijkheden maar niet daadwerkelijk vereist door een ander pakket)
- `sudo pacman -Scc` : leeg de hele `pacman` cache

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
