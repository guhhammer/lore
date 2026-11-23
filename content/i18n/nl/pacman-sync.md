# pacman-sync

Synchroniseer pakketten van externe repositories.

## Usage
- `sudo pacman -S pakket` : installeer een nieuw pakket
- `sudo pacman -Syu` : synchroniseer en ververs (y) de pakketdatabase en voer een sysupgrade uit (voeg `--downloadonly` toe om alleen de pakketten te downloaden en niet te upgraden)
- `sudo pacman -Syu --noconfirm pakket` : update en upgrade alle pakketten en installeer een nieuw pakket zonder bevestiging
- `pacman -Ss "zoekterm"` : doorzoek (s) de pakketdatabase met een `regex` of zoekwoord
- `pacman -Si pakket` : toon informatie over een pakket
- `sudo pacman -Syu --overwrite pad/naar/bestand` : overschrijf conflicterende bestanden tijdens een pakketupdate
- `sudo pacman -Sc` : verwijder niet-geïnstalleerde pakketten en ongebruikte repositories uit de cache (gebruik de vlaggen `Scc` om alle pakketten op te schonen (c))
- `sudo pacman -S pakket=versie` : specificeer de pakketversie die geïnstalleerd dient te worden

## Links
- More information: https://manned.org/pacman.8
