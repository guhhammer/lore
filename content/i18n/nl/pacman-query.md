# pacman-query

Raadpleeg de lokale pakketdatabase.

## Usage
- `pacman -Q` : query de lokale pakketdatabase en toon geïnstalleerde pakketten en versies
- `pacman -Qeq` : toon alleen pakketten die expliciet geïnstalleerd zijn zonder versienummers
- `pacman -Qo bestandsnaam` : zoek welk pakket een bestand bezit (o)
- `pacman -Qi pakket` : toon informatie over een geïnstalleerd (i) pakket
- `pacman -Ql pakket` : toon de lijst met bestanden welke een specifiek pakket bezit
- `pacman -Qdtq` : maak een lijst van pakketten welke geïnstalleerd zijn als afhankelijkheden maar niet vereist door een pakket en print in stille (q) modus (alleen pakketnaam wordt weergegeven)
- `pacman -Qm` : toon geïnstalleerde pakketten die vreemd (m) zijn aan de repositorydatabase
- `pacman -Qu` : toon pakketten die geüpgraded (u) kunnen worden

## Links
- More information: https://manned.org/pacman.8
