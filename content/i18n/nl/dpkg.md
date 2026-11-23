# dpkg

Debian pakketbeheerder. Sommige subcommando's zoals `deb` hebben hun eigen documentatie.

## Usage
- `sudo dpkg -i pad/naar/bestand.deb` : installeer een pakket
- `sudo dpkg -r pakket` : verwijder een pakket
- `dpkg -l patroon` : toon geïnstalleerde pakketten
- `dpkg -L pakket` : toon de inhoud van een pakket
- `dpkg -c pad/naar/bestand.deb` : toon de inhoud van een lokaal pakketbestand
- `dpkg -S pad/naar/bestand` : zoek uit welk pakket een bestand bezit
- `sudo dpkg -P pakket` : schoon een geïnstalleerd of al verwijderd pakket op, inclusief configuratie

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
