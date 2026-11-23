# dnf

Pakketbeheerder voor Fedora 41+ en RHEL 10.

## Usage
- `sudo dnf up` : upgrade geïnstalleerde pakketten naar de nieuwste beschikbare versies
- `dnf se sleutelwoord1 sleutelwoord2 ...` : zoek naar pakketten via sleutelwoorden
- `dnf if pakket` : toon gedetailleerde informatie over een pakket
- `sudo dnf in pakket1 pakket2 ...` : installeer nieuwe pakketten (gebruik `--assumeyes` om alle prompts automatisch te bevestigen)
- `sudo dnf rm pakket1 pakket2 ...` : verwijder pakketten
- `dnf ls --installed` : toon alle geïnstalleerde pakketten
- `dnf provides commando` : vind welke pakketten een bepaald commando leveren
- `sudo dnf clean all|dbcache|expire-cache|metadata|packages` : verwijder gecachte data

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
