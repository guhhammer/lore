# apt

Pakketbeheerder voor op Debian gebaseerde distributies. Gebruiksvriendelijk alternatief voor `apt-get` voor interactief gebruik.

## Usage
- `sudo apt update` : werk de lijst van beschikbare pakketten en versies bij (het wordt aanbevolen dit uit te voeren voor elk ander `apt` commando)
- `apt search pakket` : zoek naar pakketten op naam of beschrijving
- `apt list pakket` : zoek naar pakketten op naam (ondersteunt wildcards zoals `*`)
- `apt show pakket` : toon gedetailleerde informatie voor een specifiek pakket
- `sudo apt install pakket` : installeer een pakket, of werk het bij naar de nieuwste versie
- `sudo apt remove pakket` : verwijder een pakket (gebruik in plaats daarvan `purge` om ook de configuratiebestanden te verwijderen)
- `sudo apt upgrade` : upgrade alle geïnstalleerde pakketten naar hun nieuwste versies
- `apt list -i` : toon alle geïnstalleerde pakketten

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
