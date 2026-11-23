# pacman-database

Werk op de pakketdatabase van Arch Linux. Wijzig bepaalde attributen van de geïnstalleerde pakketten.

## Usage
- `sudo pacman -D --asdeps pakket` : markeer een pakket als impliciet geïnstalleerd
- `sudo pacman -D --asexplicit pakket` : markeer een pakket als expliciet geïnstalleerd
- `pacman -Dk` : check dat alle pakket-afhankelijkheden zijn geïnstalleerd
- `pacman -Dkk` : check de sync Database om ervoor te zorgen dat alle gespecificeerde afhankelijkheden van downloadbare pakketten beschikbaar zijn
- `pacman -Dkq` : check en toon in stille (q) modus (alleen foutmeldingen worden weergegeven)
- `pacman -Dh` : toon de help

## Links
- More information: https://manned.org/pacman.8
