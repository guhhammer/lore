# pacman-upgrade

Installeer pakketten handmatig vanuit archiefbestanden.

## Usage
- `sudo pacman -U pad/naar/pakket1.pkg.tar.zst pad/naar/pakket2.pkg.tar.zst ...` : installeer een of meerdere pakketten vanuit bestanden
- `sudo pacman -U --noconfirm pad/naar/pakket.pkg.tar.zst` : installeer een pakket zonder vragen te stellen
- `sudo pacman -U --overwrite pad/naar/bestand pad/naar/pakket.pkg.tar.zst` : overschrijf conflicterende bestanden tijdens het installeren van een pakket
- `sudo pacman -Ud pad/naar/pakket.pkg.tar.zst` : installeer een pakket en sla de controles van afhankelijkheidsversie over
- `pacman -Up pad/naar/pakket.pkg.tar.zst` : haal pakketten op en toon (p) welke beïnvloed worden door een upgrade (installeert geen pakketten)
- `pacman -Uh` : toon de help

## Links
- More information: https://manned.org/pacman.8
